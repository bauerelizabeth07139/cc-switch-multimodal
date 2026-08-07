import { describe, expect, it } from "vitest";
import {
  getModelCapability,
  isModelMultimodal,
  modelSupportsModality,
  isReasoningModel,
  getThinkingEfforts,
  getModelContextLimit,
} from "@/config/modelCapabilities";

describe("model capabilities dictionary (StepPlan verification)", () => {
  it("step-3.7-flash is multimodal (image+video), others are not", () => {
    expect(isModelMultimodal("step-3.7-flash")).toBe(true);
    expect(modelSupportsModality("step-3.7-flash", "image")).toBe(true);
    expect(modelSupportsModality("step-3.7-flash", "video")).toBe(true);

    expect(isModelMultimodal("step-3.5-flash")).toBe(false);
    expect(modelSupportsModality("step-3.5-flash", "image")).toBe(false);
    expect(isModelMultimodal("step-3.5-flash-2603")).toBe(false);
    expect(isModelMultimodal("step-2x-large")).toBe(false);
    expect(isModelMultimodal("step-1o")).toBe(false);
  });

  it("same name is same model regardless of URL/provider prefix", () => {
    expect(getModelCapability("stepfun/step-3.7-flash")).toEqual(
      getModelCapability("step-3.7-flash"),
    );
    expect(getModelCapability("openai/gpt-4o")).toEqual(
      getModelCapability("gpt-4o"),
    );
    expect(isModelMultimodal("google/gemini-2.5-flash")).toBe(true);
    expect(isModelMultimodal("deepseek/deepseek-v4-flash")).toBe(false);
  });

  it("reasoning / thinking effort / context lookups", () => {
    expect(isReasoningModel("gpt-5.4")).toBe(true);
    expect(isReasoningModel("gpt-4o")).toBe(false);
    expect(getThinkingEfforts("step-3.7-flash")).toEqual([
      "low",
      "medium",
      "high",
    ]);
    expect(getModelContextLimit("gpt-4o")).toBeGreaterThan(0);
    expect(getModelContextLimit("deepseek-chat")).toBe(163840);
  });

  it("claude-3-opus is multimodal (image capable) per authoritative data", () => {
    expect(isModelMultimodal("claude-3-opus")).toBe(true);
  });
});
