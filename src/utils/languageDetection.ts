import { ModelOperations } from "@vscode/vscode-languagedetection";
import { readTextFile, readFile, BaseDirectory } from "@tauri-apps/plugin-fs";
import { message } from "ant-design-vue";
import log from "./logger";

const loadModel = async () => {
  let model = await readTextFile("resources/detect_lang_model/model.json", {
    baseDir: BaseDirectory.Resource,
  });

  return JSON.parse(model);
};

const loadWeight = async () => {
  const weight = await readFile(
    "resources/detect_lang_model/group1-shard1of1.bin",
    {
      baseDir: BaseDirectory.Resource,
    },
  );
  return weight.buffer;
};

export async function detect(code: string): Promise<string> {
  try {
    const modelOperations = new ModelOperations({
      modelJsonLoaderFunc: loadModel,
      weightsLoaderFunc: loadWeight,
    });
    const result = await modelOperations.runModel(code);
    return result[0].languageId;
  } catch (err) {
    const errMsg = `检测代码语言失败：${err}`;
    log(errMsg, "error");
    message.error(errMsg);
    return "";
  }
}
