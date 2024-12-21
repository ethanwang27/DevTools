import { MenuProps } from "ant-design-vue";

export const Menus: MenuProps["items"] = [
  {
    type: "group",
    label: "条形码/二维码",
    children: [
      {
        label: "二维码生成",
        key: "generateQrCode",
      },
    ],
  },
  {
    type: "group",
    label: "数据生成",
    children: [
      {
        label: "身份证号码生成",
        key: "generateIdNo",
      },
    ],
  },
  {
    type: "group",
    label: "开发工具",
    children: [
      {
        label: "Unicode编码转换",
        key: "unicode",
      },
      {
        label: "正则表达式",
        key: "regexp",
      },
    ],
  },
  {
    type: "group",
    label: "Encoder/Decoder",
    children: [
      {
        label: "Base64",
        key: "base64",
      },
      {
        label: "Hash生成",
        key: "hashGenerator",
      },
      {
        label: "URL编码/解码",
        key: "urlEncoding",
      },
      {
        label: "URL解析",
        key: "urlParser",
      },
    ],
  },
  {
    type: "group",
    label: "Markdown",
    children: [
      {
        label: "Markdown预览",
        key: "previewMarkdown",
      },
    ],
  },
];
