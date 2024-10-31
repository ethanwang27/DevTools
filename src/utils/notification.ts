import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from "@tauri-apps/plugin-notification";

/**
 * 发送消息
 * @param title 消息标题
 * @param body 消息内容
 */
export default async function notify(title: string, body: string) {
  let granted = await isPermissionGranted();
  if (!granted) {
    const result = await requestPermission();
    granted = result === "granted";
  }
  if (granted) {
    sendNotification({ title, body });
  }
}
