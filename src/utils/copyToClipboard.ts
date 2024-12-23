import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { message } from "ant-design-vue";

export async function copyText(data: string) {
  await writeText(data);
  message.success("复制成功");
}
