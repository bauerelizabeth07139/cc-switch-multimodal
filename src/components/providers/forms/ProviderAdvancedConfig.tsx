import { useTranslation } from "react-i18next";
import { useState, useEffect, useMemo } from "react";
import {
  ChevronDown,
  ChevronRight,
  Coins,
  Eye,
  Brain,
  SlidersHorizontal,
} from "lucide-react";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { cn } from "@/lib/utils";
import { settingsApi } from "@/lib/api";
import type { Provider } from "@/types";
import {
  getModelCapability,
  normalizeModelName,
} from "@/config/modelCapabilities";
export type PricingModelSourceOption = "inherit" | "request" | "response";

interface ProviderPricingConfig {
  enabled: boolean;
  costMultiplier?: string;
  pricingModelSource: PricingModelSourceOption;
}

export interface MultimodalBindingConfig {
  enabled: boolean;
  name: string;
  role: "eyes" | "brain";
  counterpartModel: string;
  counterpartProviderId: string;
}

/** 模型能力覆盖配置（推理模型 / 思考强度档位 / 上下文上限） */
export interface ModelCapabilitiesConfig {
  modelName: string;
  reasoning?: boolean;
  thinkingEffort?: string;
  contextLimit?: number;
}

interface ProviderAdvancedConfigProps {
  pricingConfig: ProviderPricingConfig;
  onPricingConfigChange: (config: ProviderPricingConfig) => void;
  multimodalBinding?: MultimodalBindingConfig;
  onMultimodalBindingChange?: (config: MultimodalBindingConfig | null) => void;
  allProviders?: Record<string, Provider>;
  currentProviderId?: string;
  modelCapabilities?: ModelCapabilitiesConfig;
  onModelCapabilitiesChange?: (config: ModelCapabilitiesConfig) => void;
}

export function ProviderAdvancedConfig({
  pricingConfig,
  onPricingConfigChange,
  multimodalBinding,
  onMultimodalBindingChange,
  allProviders = {},
  currentProviderId,
  modelCapabilities,
  onModelCapabilitiesChange,
}: ProviderAdvancedConfigProps) {
  const { t } = useTranslation();
  const [isPricingConfigOpen, setIsPricingConfigOpen] = useState(
    pricingConfig.enabled,
  );

  const [isMultimodalOpen, setIsMultimodalOpen] = useState(false);
  const [localBinding, setLocalBinding] = useState<MultimodalBindingConfig>(
    () =>
      multimodalBinding ?? {
        enabled: false,
        name: "",
        role: "eyes",
        counterpartModel: "",
        counterpartProviderId: "",
      },
  );
  const [isSavingBinding, setIsSavingBinding] = useState(false);

  const [isCapabilitiesOpen, setIsCapabilitiesOpen] = useState(false);
  const [localCapabilities, setLocalCapabilities] =
    useState<ModelCapabilitiesConfig>(
      () =>
        modelCapabilities ?? {
          modelName: "",
          reasoning: undefined,
          thinkingEffort: undefined,
          contextLimit: undefined,
        },
    );

  useEffect(() => {
    setIsPricingConfigOpen(pricingConfig.enabled);
  }, [pricingConfig.enabled]);

  useEffect(() => {
    if (multimodalBinding) {
      setLocalBinding(multimodalBinding);
    }
  }, [multimodalBinding]);

  useEffect(() => {
    if (modelCapabilities) {
      setLocalCapabilities(modelCapabilities);
    }
  }, [modelCapabilities]);

  const dictionaryCapability = useMemo(() => {
    if (!localCapabilities.modelName.trim()) return undefined;
    return getModelCapability(localCapabilities.modelName.trim());
  }, [localCapabilities.modelName]);

  const normalizedModel = normalizeModelName(
    localCapabilities.modelName.trim() || "",
  );

  const handleCapabilitiesChange = (patch: Partial<ModelCapabilitiesConfig>) => {
    const next = { ...localCapabilities, ...patch };
    setLocalCapabilities(next);
    onModelCapabilitiesChange?.(next);
  };

  const effortOptions = useMemo(() => {
    if (dictionaryCapability?.thinkingEffort?.length) {
      return dictionaryCapability.thinkingEffort;
    }
    return ["low", "medium", "high"];
  }, [dictionaryCapability]);

  const handleSaveBinding = async () => {
    if (!localBinding.enabled) {
      onMultimodalBindingChange?.(null);
      return;
    }
    if (!localBinding.name.trim() || !localBinding.counterpartModel.trim()) {
      return;
    }
    if (!currentProviderId) {
      // 添加模式：供应商尚未创建，仅上报配置，由 AddProviderDialog 在创建后持久化
      onMultimodalBindingChange?.({ ...localBinding });
      return;
    }
    setIsSavingBinding(true);
    try {
      await settingsApi.addCompositeModel({
        name: localBinding.name.trim(),
        eyes_model:
          localBinding.role === "eyes"
            ? localBinding.counterpartModel.trim()
            : "",
        eyes_provider_id:
          localBinding.role === "eyes"
            ? localBinding.counterpartProviderId
            : currentProviderId || "",
        brain_model:
          localBinding.role === "brain"
            ? localBinding.counterpartModel.trim()
            : "",
        brain_provider_id:
          localBinding.role === "brain"
            ? localBinding.counterpartProviderId
            : currentProviderId || "",
      });
      onMultimodalBindingChange?.({ ...localBinding });
    } catch (e) {
      console.error("Failed to add composite model binding:", e);
    } finally {
      setIsSavingBinding(false);
    }
  };

  const counterpartProviders = useMemo(() => {
    if (!allProviders || !currentProviderId) return Object.values(allProviders);
    return Object.values(allProviders).filter((p) => p.id !== currentProviderId);
  }, [allProviders, currentProviderId]);

  return (
    <div className="space-y-4">
      {/* 计费配置 */}
      <div className="rounded-lg border border-border/50 bg-muted/20">
        <button
          type="button"
          className="flex w-full items-center justify-between p-4 hover:bg-muted/30 transition-colors"
          onClick={() => setIsPricingConfigOpen(!isPricingConfigOpen)}
        >
          <div className="flex items-center gap-3">
            <Coins className="h-4 w-4 text-muted-foreground" />
            <span className="font-medium">
              {t("providerAdvanced.pricingConfig", {
                defaultValue: "计费配置",
              })}
            </span>
          </div>
          <div className="flex items-center gap-3">
            <div
              className="flex items-center gap-2"
              onClick={(e) => e.stopPropagation()}
            >
              <Label
                htmlFor="pricing-config-enabled"
                className="text-sm text-muted-foreground"
              >
                {t("providerAdvanced.useCustomPricing", {
                  defaultValue: "使用单独配置",
                })}
              </Label>
              <Switch
                id="pricing-config-enabled"
                checked={pricingConfig.enabled}
                onCheckedChange={(checked) => {
                  onPricingConfigChange({ ...pricingConfig, enabled: checked });
                  if (checked) setIsPricingConfigOpen(true);
                }}
              />
            </div>
            {isPricingConfigOpen ? (
              <ChevronDown className="h-4 w-4 text-muted-foreground" />
            ) : (
              <ChevronRight className="h-4 w-4 text-muted-foreground" />
            )}
          </div>
        </button>
        <div
          className={cn(
            "overflow-hidden transition-all duration-200",
            isPricingConfigOpen
              ? "max-h-[500px] opacity-100"
              : "max-h-0 opacity-0",
          )}
        >
          <div className="border-t border-border/50 p-4 space-y-4">
            <p className="text-sm text-muted-foreground">
              {t("providerAdvanced.pricingConfigDesc", {
                defaultValue:
                  "为此供应商配置单独的计费参数，不启用时使用全局默认配置。",
              })}
            </p>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="space-y-2">
                <Label htmlFor="cost-multiplier">
                  {t("providerAdvanced.costMultiplier", {
                    defaultValue: "成本倍率",
                  })}
                </Label>
                <Input
                  id="cost-multiplier"
                  type="number"
                  step="0.01"
                  min="0"
                  inputMode="decimal"
                  value={pricingConfig.costMultiplier || ""}
                  onChange={(e) =>
                    onPricingConfigChange({
                      ...pricingConfig,
                      costMultiplier: e.target.value || undefined,
                    })
                  }
                  placeholder={t("providerAdvanced.costMultiplierPlaceholder", {
                    defaultValue: "留空使用全局默认（1）",
                  })}
                  disabled={!pricingConfig.enabled}
                />
                <p className="text-xs text-muted-foreground">
                  {t("providerAdvanced.costMultiplierHint", {
                    defaultValue: "实际成本 = 基础成本 × 倍率，支持小数如 1.5",
                  })}
                </p>
              </div>
              <div className="space-y-2">
                <Label htmlFor="pricing-model-source">
                  {t("providerAdvanced.pricingModelSourceLabel", {
                    defaultValue: "计费模式",
                  })}
                </Label>
                <Select
                  value={pricingConfig.pricingModelSource}
                  onValueChange={(value) =>
                    onPricingConfigChange({
                      ...pricingConfig,
                      pricingModelSource: value as PricingModelSourceOption,
                    })
                  }
                  disabled={!pricingConfig.enabled}
                >
                  <SelectTrigger id="pricing-model-source">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="inherit">
                      {t("providerAdvanced.pricingModelSourceInherit", {
                        defaultValue: "继承全局默认",
                      })}
                    </SelectItem>
                    <SelectItem value="request">
                      {t("providerAdvanced.pricingModelSourceRequest", {
                        defaultValue: "请求模型",
                      })}
                    </SelectItem>
                    <SelectItem value="response">
                      {t("providerAdvanced.pricingModelSourceResponse", {
                        defaultValue: "返回模型",
                      })}
                    </SelectItem>
                  </SelectContent>
                </Select>
                <p className="text-xs text-muted-foreground">
                  {t("providerAdvanced.pricingModelSourceHint", {
                    defaultValue: "选择按请求模型还是返回模型进行定价匹配",
                  })}
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* 模型能力配置 */}
      <div className="rounded-lg border border-border/50 bg-muted/20">
        <button
          type="button"
          className="flex w-full items-center justify-between p-4 hover:bg-muted/30 transition-colors"
          onClick={() => setIsCapabilitiesOpen(!isCapabilitiesOpen)}
        >
          <div className="flex items-center gap-3">
            <SlidersHorizontal className="h-4 w-4 text-violet-500" />
            <span className="font-medium">
              {t("providerAdvanced.modelCapabilities.title", {
                defaultValue: "模型能力配置",
              })}
            </span>
          </div>
          <div className="flex items-center gap-3">
            {isCapabilitiesOpen ? (
              <ChevronDown className="h-4 w-4 text-muted-foreground" />
            ) : (
              <ChevronRight className="h-4 w-4 text-muted-foreground" />
            )}
          </div>
        </button>
        <div
          className={cn(
            "overflow-hidden transition-all duration-200",
            isCapabilitiesOpen
              ? "max-h-[600px] opacity-100"
              : "max-h-0 opacity-0",
          )}
        >
          <div className="border-t border-border/50 p-4 space-y-4">
            <p className="text-sm text-muted-foreground">
              {t("providerAdvanced.modelCapabilities.description", {
                defaultValue:
                  "输入模型名称后，根据内置能力字典自动识别是否为多模态、是否推理模型、支持的思考强度档位与上下文上限；可手动覆盖。",
              })}
            </p>

            <div className="space-y-2">
              <Label htmlFor="capability-model-name">
                {t("providerAdvanced.modelCapabilities.modelName", {
                  defaultValue: "模型名称",
                })}
              </Label>
              <Input
                id="capability-model-name"
                value={localCapabilities.modelName}
                onChange={(e) =>
                  handleCapabilitiesChange({ modelName: e.target.value })
                }
                placeholder="例: gpt-4o / step-3.7-flash / deepseek-chat"
                autoComplete="off"
              />
            </div>

            {dictionaryCapability && (
              <div className="rounded-md border border-border/50 bg-background/50 p-3 space-y-2">
                <div className="flex items-center justify-between">
                  <span className="text-sm font-medium">
                    {t("providerAdvanced.modelCapabilities.detected", {
                      defaultValue: "字典识别结果",
                    })}{" "}
                    <code className="text-xs bg-muted px-1.5 py-0.5 rounded">
                      {normalizedModel}
                    </code>
                  </span>
                  <span className="text-xs text-muted-foreground">
                    {dictionaryCapability.modalities.length > 1
                      ? t(
                          "providerAdvanced.modelCapabilities.multimodal",
                          { defaultValue: "多模态模型" },
                        )
                      : t("providerAdvanced.modelCapabilities.textOnly", {
                          defaultValue: "纯文本模型",
                        })}
                  </span>
                </div>
                <div className="flex flex-wrap gap-1.5">
                  {dictionaryCapability.modalities.map((m) => (
                    <span
                      key={m}
                      className="text-xs rounded-full border border-border/50 px-2 py-0.5"
                    >
                      {m}
                    </span>
                  ))}
                </div>
                <div className="grid grid-cols-2 gap-3 pt-1">
                  <div className="text-xs text-muted-foreground">
                    {t("providerAdvanced.modelCapabilities.reasoning", {
                      defaultValue: "推理模型",
                    })}
                    :{" "}
                    <span className="text-foreground">
                      {dictionaryCapability.reasoning ? "✓" : "—"}
                    </span>
                  </div>
                  <div className="text-xs text-muted-foreground">
                    {t("providerAdvanced.modelCapabilities.context", {
                      defaultValue: "上下文上限",
                    })}
                    :{" "}
                    <span className="text-foreground">
                      {dictionaryCapability.context > 0
                        ? `${(dictionaryCapability.context / 1000).toFixed(0)}K`
                        : "—"}
                    </span>
                  </div>
                </div>
                {dictionaryCapability.thinkingEffort && (
                  <div className="text-xs text-muted-foreground">
                    {t("providerAdvanced.modelCapabilities.effortLevels", {
                      defaultValue: "思考强度档位",
                    })}
                    :{" "}
                    <span className="text-foreground">
                      {dictionaryCapability.thinkingEffort.join(" / ")}
                    </span>
                  </div>
                )}
              </div>
            )}

            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div className="space-y-2">
                <Label htmlFor="capability-reasoning">
                  {t("providerAdvanced.modelCapabilities.isReasoning", {
                    defaultValue: "推理模型",
                  })}
                </Label>
                <Switch
                  id="capability-reasoning"
                  checked={localCapabilities.reasoning ?? false}
                  onCheckedChange={(checked) =>
                    handleCapabilitiesChange({ reasoning: checked })
                  }
                />
              </div>
              <div className="space-y-2">
                <Label htmlFor="capability-effort">
                  {t("providerAdvanced.modelCapabilities.effort", {
                    defaultValue: "思考强度档位",
                  })}
                </Label>
                <Select
                  value={localCapabilities.thinkingEffort || undefined}
                  onValueChange={(value) =>
                    handleCapabilitiesChange({
                      thinkingEffort: value === "inherit" ? undefined : value,
                    })
                  }
                >
                  <SelectTrigger id="capability-effort" className="h-9">
                    <SelectValue
                      placeholder={t(
                        "providerAdvanced.modelCapabilities.effortInherit",
                        { defaultValue: "继承字典默认" },
                      )}
                    />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="inherit">
                      {t(
                        "providerAdvanced.modelCapabilities.effortInherit",
                        { defaultValue: "继承字典默认" },
                      )}
                    </SelectItem>
                    {effortOptions.map((effort) => (
                      <SelectItem key={effort} value={effort}>
                        {effort}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
              <div className="space-y-2">
                <Label htmlFor="capability-context">
                  {t("providerAdvanced.modelCapabilities.contextLimit", {
                    defaultValue: "上下文上限 (tokens)",
                  })}
                </Label>
                <Input
                  id="capability-context"
                  type="number"
                  min={0}
                  inputMode="numeric"
                  value={localCapabilities.contextLimit ?? ""}
                  onChange={(e) =>
                    handleCapabilitiesChange({
                      contextLimit:
                        e.target.value === ""
                          ? undefined
                          : Number(e.target.value),
                    })
                  }
                  placeholder={
                    dictionaryCapability?.context
                      ? String(dictionaryCapability.context)
                      : "例: 200000"
                  }
                />
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* 多模态模型绑定 */}
      <div className="rounded-lg border border-border/50 bg-muted/20">
        <button
          type="button"
          className="flex w-full items-center justify-between p-4 hover:bg-muted/30 transition-colors"
          onClick={() => setIsMultimodalOpen(!isMultimodalOpen)}
        >
          <div className="flex items-center gap-3">
            <Eye className="h-4 w-4 text-pink-500" />
            <span className="font-medium">
              {t("providerAdvanced.multimodalBinding.title", {
                defaultValue: "多模态模型绑定",
              })}
            </span>
          </div>
          <div className="flex items-center gap-3">
            <div
              className="flex items-center gap-2"
              onClick={(e) => e.stopPropagation()}
            >
              <Label
                htmlFor="multimodal-binding-enabled"
                className="text-sm text-muted-foreground"
              >
                {t("providerAdvanced.multimodalBinding.enable", {
                  defaultValue: "启用",
                })}
              </Label>
              <Switch
                id="multimodal-binding-enabled"
                checked={localBinding.enabled}
                onCheckedChange={(checked) => {
                  const next = { ...localBinding, enabled: checked };
                  setLocalBinding(next);
                  if (checked) setIsMultimodalOpen(true);
                  onMultimodalBindingChange?.(checked ? next : null);
                }}
              />
            </div>
            {isMultimodalOpen ? (
              <ChevronDown className="h-4 w-4 text-muted-foreground" />
            ) : (
              <ChevronRight className="h-4 w-4 text-muted-foreground" />
            )}
          </div>
        </button>
        <div
          className={cn(
            "overflow-hidden transition-all duration-200",
            isMultimodalOpen
              ? "max-h-[800px] opacity-100"
              : "max-h-0 opacity-0",
          )}
        >
          <div className="border-t border-border/50 p-4 space-y-4">
            <p className="text-sm text-muted-foreground">
              {t("providerAdvanced.multimodalBinding.description", {
                defaultValue:
                  "将此供应商的模型与另一个模型绑定为复合模型，让多模态模型作为眼睛，单模态模型作为大脑。",
              })}
            </p>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="space-y-2">
                <Label htmlFor="composite-name">
                  {t("providerAdvanced.multimodalBinding.name", {
                    defaultValue: "复合模型名称",
                  })}
                </Label>
                <Input
                  id="composite-name"
                  value={localBinding.name}
                  onChange={(e) =>
                    setLocalBinding((prev) => ({ ...prev, name: e.target.value }))
                  }
                  placeholder={t(
                    "providerAdvanced.multimodalBinding.namePlaceholder",
                    { defaultValue: "例: my-vision-reasoning" },
                  )}
                  disabled={!localBinding.enabled || isSavingBinding}
                />
              </div>
              <div className="space-y-2">
                <Label htmlFor="binding-role">
                  {t("providerAdvanced.multimodalBinding.role", {
                    defaultValue: "当前供应商角色",
                  })}
                </Label>
                <Select
                  value={localBinding.role}
                  onValueChange={(value) =>
                    setLocalBinding((prev) => ({
                      ...prev,
                      role: value as "eyes" | "brain",
                    }))
                  }
                  disabled={!localBinding.enabled || isSavingBinding}
                >
                  <SelectTrigger id="binding-role">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="eyes">
                      <div className="flex items-center gap-2">
                        <Eye className="h-4 w-4" />
                        {t("providerAdvanced.multimodalBinding.roleEyes", {
                          defaultValue: "眼睛（多模态）",
                        })}
                      </div>
                    </SelectItem>
                    <SelectItem value="brain">
                      <div className="flex items-center gap-2">
                        <Brain className="h-4 w-4" />
                        {t("providerAdvanced.multimodalBinding.roleBrain", {
                          defaultValue: "大脑（推理）",
                        })}
                      </div>
                    </SelectItem>
                  </SelectContent>
                </Select>
              </div>
              <div className="space-y-2">
                <Label htmlFor="counterpart-model">
                  {localBinding.role === "eyes"
                    ? t("providerAdvanced.multimodalBinding.brainModel", {
                        defaultValue: "大脑模型名称",
                      })
                    : t("providerAdvanced.multimodalBinding.eyesModel", {
                        defaultValue: "眼睛模型名称",
                      })}
                </Label>
                <Input
                  id="counterpart-model"
                  value={localBinding.counterpartModel}
                  onChange={(e) =>
                    setLocalBinding((prev) => ({
                      ...prev,
                      counterpartModel: e.target.value,
                    }))
                  }
                  placeholder={t(
                    "providerAdvanced.multimodalBinding.modelPlaceholder",
                    { defaultValue: "例: gpt-4o" },
                  )}
                  disabled={!localBinding.enabled || isSavingBinding}
                />
              </div>
              <div className="space-y-2">
                <Label htmlFor="counterpart-provider">
                  {localBinding.role === "eyes"
                    ? t("providerAdvanced.multimodalBinding.brainProvider", {
                        defaultValue: "大脑供应商",
                      })
                    : t("providerAdvanced.multimodalBinding.eyesProvider", {
                        defaultValue: "眼睛供应商",
                      })}
                </Label>
                <Select
                  value={localBinding.counterpartProviderId || undefined}
                  onValueChange={(value) =>
                    setLocalBinding((prev) => ({
                      ...prev,
                      counterpartProviderId: value,
                    }))
                  }
                  disabled={
                    !localBinding.enabled ||
                    isSavingBinding ||
                    counterpartProviders.length === 0
                  }
                >
                  <SelectTrigger id="counterpart-provider">
                    <SelectValue
                      placeholder={t(
                        "providerAdvanced.multimodalBinding.selectProvider",
                        { defaultValue: "选择供应商" },
                      )}
                    />
                  </SelectTrigger>
                  <SelectContent>
                    {counterpartProviders.map((p) => (
                      <SelectItem key={p.id} value={p.id}>
                        {p.name}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
            </div>
            <div className="flex items-center gap-2">
              <Button
                size="sm"
                onClick={handleSaveBinding}
                disabled={
                  !localBinding.enabled ||
                  isSavingBinding ||
                  !localBinding.name.trim() ||
                  !localBinding.counterpartModel.trim() ||
                  !localBinding.counterpartProviderId
                }
              >
                {isSavingBinding
                  ? t("common.saving", { defaultValue: "保存中..." })
                  : t("providerAdvanced.multimodalBinding.save", {
                      defaultValue: "保存绑定",
                    })}
              </Button>
              {localBinding.enabled && (
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={() => {
                    const cleared = {
                      enabled: false,
                      name: "",
                      role: "eyes" as const,
                      counterpartModel: "",
                      counterpartProviderId: "",
                    };
                    setLocalBinding(cleared);
                    onMultimodalBindingChange?.(null);
                  }}
                  disabled={isSavingBinding}
                >
                  {t("common.reset", { defaultValue: "重置" })}
                </Button>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
