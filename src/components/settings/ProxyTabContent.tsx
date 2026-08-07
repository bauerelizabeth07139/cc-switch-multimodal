import { useState } from "react";
import {
  Server,
  Activity,
  Zap,
  Globe,
  ShieldAlert,
  ScanEye,
} from "lucide-react";
import { motion } from "framer-motion";
import { useTranslation } from "react-i18next";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Badge } from "@/components/ui/badge";
import { ProxyPanel } from "@/components/proxy";
import { AutoFailoverConfigPanel } from "@/components/proxy/AutoFailoverConfigPanel";
import { FailoverQueueManager } from "@/components/proxy/FailoverQueueManager";
import { RectifierConfigPanel } from "@/components/settings/RectifierConfigPanel";
import { GlobalProxySettings } from "@/components/settings/GlobalProxySettings";
import { MultimodalSettingsPanel } from "@/components/settings/MultimodalSettingsPanel";
import { ConfirmDialog } from "@/components/ConfirmDialog";
import { ToggleRow } from "@/components/ui/toggle-row";
import { useProxyStatus } from "@/hooks/useProxyStatus";
import type { SettingsFormState } from "@/hooks/useSettings";

interface ProxyTabContentProps {
  settings: SettingsFormState;
  onAutoSave: (updates: Partial<SettingsFormState>) => Promise<boolean | void>;
}

export const FAILOVER_APPS = [
  { id: "claude", label: "Claude" },
  { id: "codex", label: "Codex" },
  { id: "gemini", label: "Gemini" },
  { id: "grokbuild", label: "Grok Build" },
] as const;

export function ProxyTabContent({
  settings,
  onAutoSave,
}: ProxyTabContentProps) {
  const { t } = useTranslation();
  const [showProxyConfirm, setShowProxyConfirm] = useState(false);
  const [showFailoverConfirm, setShowFailoverConfirm] = useState(false);
  const [activeSubTab, setActiveSubTab] = useState("proxy");

  const {
    isRunning,
    takeoverStatus,
    startProxyServer,
    stopWithRestore,
    isPending: isProxyPending,
  } = useProxyStatus();

  const handleToggleProxy = async (checked: boolean) => {
    try {
      if (!checked) {
        await stopWithRestore();
      } else if (!settings?.proxyConfirmed) {
        setShowProxyConfirm(true);
      } else {
        await startProxyServer();
      }
    } catch (error) {
      console.error("Toggle proxy failed:", error);
    }
  };

  const handleProxyConfirm = async () => {
    setShowProxyConfirm(false);
    try {
      await onAutoSave({ proxyConfirmed: true });
      await startProxyServer();
    } catch (error) {
      console.error("Proxy confirm failed:", error);
    }
  };

  const handleFailoverToggleChange = (checked: boolean) => {
    if (checked && !settings?.failoverConfirmed) {
      setShowFailoverConfirm(true);
    } else {
      void onAutoSave({ enableFailoverToggle: checked });
    }
  };

  const handleFailoverConfirm = async () => {
    setShowFailoverConfirm(false);
    try {
      await onAutoSave({ failoverConfirmed: true, enableFailoverToggle: true });
    } catch (error) {
      console.error("Failover confirm failed:", error);
    }
  };

  return (
    <motion.div
      initial={{ opacity: 0, y: 10 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.3 }}
      className="space-y-4"
    >
      <Tabs value={activeSubTab} onValueChange={setActiveSubTab} className="w-full">
        <TabsList className="grid w-full grid-cols-5 glass rounded-lg">
          <TabsTrigger value="proxy">
            <Server className="h-3.5 w-3.5 mr-1.5" />
            {t("settings.advanced.proxy.title")}
          </TabsTrigger>
          <TabsTrigger value="failover">
            <Activity className="h-3.5 w-3.5 mr-1.5" />
            {t("settings.advanced.failover.title")}
          </TabsTrigger>
          <TabsTrigger value="rectifier">
            <Zap className="h-3.5 w-3.5 mr-1.5" />
            {t("settings.advanced.rectifier.title")}
          </TabsTrigger>
          <TabsTrigger value="multimodal">
            <ScanEye className="h-3.5 w-3.5 mr-1.5" />
            {t("settings.advanced.multimodal.title", {
              defaultValue: "多模态路由",
            })}
          </TabsTrigger>
          <TabsTrigger value="globalProxy">
            <Globe className="h-3.5 w-3.5 mr-1.5" />
            {t("settings.advanced.globalProxy.title")}
          </TabsTrigger>
        </TabsList>

        {/* Local Proxy */}
        <TabsContent value="proxy" className="mt-4 space-y-4">
          <div className="rounded-xl glass-card overflow-hidden">
            <div className="px-6 py-4 flex items-center justify-between border-b border-border/50">
              <div className="flex items-center gap-3">
                <Server className="h-5 w-5 text-green-500" />
                <div className="text-left">
                  <h3 className="text-base font-semibold">
                    {t("settings.advanced.proxy.title")}
                  </h3>
                  <p className="text-sm text-muted-foreground font-normal">
                    {t("settings.advanced.proxy.description")}
                  </p>
                </div>
              </div>
              <Badge
                variant={isRunning ? "default" : "secondary"}
                className="gap-1.5 h-6 ml-auto mr-2"
              >
                <Activity
                  className={`h-3 w-3 ${isRunning ? "status-heartbeat" : ""}`}
                />
                {isRunning
                  ? t("settings.advanced.proxy.running")
                  : t("settings.advanced.proxy.stopped")}
              </Badge>
            </div>
            <div className="px-6 pb-6 pt-4">
              <ProxyPanel
                enableLocalProxy={settings?.enableLocalProxy ?? false}
                onEnableLocalProxyChange={(checked) =>
                  onAutoSave({ enableLocalProxy: checked })
                }
                onToggleProxy={handleToggleProxy}
                isProxyPending={isProxyPending}
              />
            </div>
          </div>
        </TabsContent>

        {/* Auto Failover */}
        <TabsContent value="failover" className="mt-4 space-y-4">
          <div className="rounded-xl glass-card overflow-hidden">
            <div className="px-6 py-4 border-b border-border/50">
              <div className="flex items-center gap-3">
                <Activity className="h-5 w-5 text-orange-500" />
                <div className="text-left">
                  <h3 className="text-base font-semibold">
                    {t("settings.advanced.failover.title")}
                  </h3>
                  <p className="text-sm text-muted-foreground font-normal">
                    {t("settings.advanced.failover.description")}
                  </p>
                </div>
              </div>
            </div>
            <div className="px-6 py-4 space-y-6">
              <ToggleRow
                icon={<ShieldAlert className="h-4 w-4 text-orange-500" />}
                title={t("settings.advanced.proxy.enableFailoverToggle")}
                description={t(
                  "settings.advanced.proxy.enableFailoverToggleDescription",
                )}
                checked={settings?.enableFailoverToggle ?? false}
                onCheckedChange={handleFailoverToggleChange}
              />

              {!isRunning && (
                <div className="p-4 rounded-lg bg-yellow-500/10 border border-yellow-500/20">
                  <p className="text-sm text-yellow-600 dark:text-yellow-400">
                    {t("proxy.failover.proxyRequired", {
                      defaultValue: "需要先启动代理服务才能配置故障转移",
                    })}
                  </p>
                </div>
              )}

              <Tabs defaultValue="claude" className="w-full">
                <TabsList className="grid w-full grid-cols-4">
                  {FAILOVER_APPS.map(({ id, label }) => (
                    <TabsTrigger key={id} value={id}>
                      {label}
                    </TabsTrigger>
                  ))}
                </TabsList>
                {FAILOVER_APPS.map(({ id: appType }) => {
                  const failoverDisabled =
                    !isRunning || !(takeoverStatus?.[appType] ?? false);
                  return (
                    <TabsContent
                      key={appType}
                      value={appType}
                      className="mt-4 space-y-6"
                    >
                      <div className="space-y-4">
                        <div>
                          <h4 className="text-sm font-semibold">
                            {t("proxy.failoverQueue.title")}
                          </h4>
                          <p className="text-xs text-muted-foreground">
                            {t("proxy.failoverQueue.description")}
                          </p>
                        </div>
                        <FailoverQueueManager
                          appType={appType}
                          disabled={failoverDisabled}
                        />
                      </div>
                      <div className="border-t border-border/50 pt-6">
                        <AutoFailoverConfigPanel
                          appType={appType}
                          disabled={failoverDisabled}
                        />
                      </div>
                    </TabsContent>
                  );
                })}
              </Tabs>
            </div>
          </div>
        </TabsContent>

        {/* Rectifier */}
        <TabsContent value="rectifier" className="mt-4 space-y-4">
          <div className="rounded-xl glass-card overflow-hidden">
            <div className="px-6 py-4 border-b border-border/50">
              <div className="flex items-center gap-3">
                <Zap className="h-5 w-5 text-purple-500" />
                <div className="text-left">
                  <h3 className="text-base font-semibold">
                    {t("settings.advanced.rectifier.title")}
                  </h3>
                  <p className="text-sm text-muted-foreground font-normal">
                    {t("settings.advanced.rectifier.description")}
                  </p>
                </div>
              </div>
            </div>
            <div className="px-6 pb-6 pt-4">
              <RectifierConfigPanel />
            </div>
          </div>
        </TabsContent>

        {/* Multimodal Routing */}
        <TabsContent value="multimodal" className="mt-4 space-y-4">
          <div className="rounded-xl glass-card overflow-hidden">
            <div className="px-6 py-4 border-b border-border/50">
              <div className="flex items-center gap-3">
                <ScanEye className="h-5 w-5 text-pink-500" />
                <div className="text-left">
                  <h3 className="text-base font-semibold">
                    {t("settings.advanced.multimodal.title", {
                      defaultValue: "多模态路由",
                    })}
                  </h3>
                  <p className="text-sm text-muted-foreground font-normal">
                    {t("settings.advanced.multimodal.description", {
                      defaultValue:
                        "配置多模态输入的自动路由和组合模型绑定",
                    })}
                  </p>
                </div>
              </div>
            </div>
            <div className="px-6 pb-6 pt-4">
              <MultimodalSettingsPanel />
            </div>
          </div>
        </TabsContent>

        {/* Global Outbound Proxy */}
        <TabsContent value="globalProxy" className="mt-4 space-y-4">
          <div className="rounded-xl glass-card overflow-hidden">
            <div className="px-6 py-4 border-b border-border/50">
              <div className="flex items-center gap-3">
                <Globe className="h-5 w-5 text-cyan-500" />
                <div className="text-left">
                  <h3 className="text-base font-semibold">
                    {t("settings.advanced.globalProxy.title")}
                  </h3>
                  <p className="text-sm text-muted-foreground font-normal">
                    {t("settings.advanced.globalProxy.description")}
                  </p>
                </div>
              </div>
            </div>
            <div className="px-6 pb-6 pt-4">
              <GlobalProxySettings />
            </div>
          </div>
        </TabsContent>
      </Tabs>

      <ConfirmDialog
        isOpen={showProxyConfirm}
        variant="info"
        title={t("confirm.proxy.title")}
        message={t("confirm.proxy.message")}
        confirmText={t("confirm.proxy.confirm")}
        onConfirm={() => void handleProxyConfirm()}
        onCancel={() => setShowProxyConfirm(false)}
      />

      <ConfirmDialog
        isOpen={showFailoverConfirm}
        variant="info"
        title={t("confirm.failover.title")}
        message={t("confirm.failover.message")}
        confirmText={t("confirm.failover.confirm")}
        onConfirm={() => void handleFailoverConfirm()}
        onCancel={() => setShowFailoverConfirm(false)}
      />
    </motion.div>
  );
}
