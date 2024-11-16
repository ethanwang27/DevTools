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
        key: "fakeIdNo",
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
    label: "Markdown",
    children: [
      {
        label: "Markdown预览",
        key: "previewMarkdown",
      },
    ],
  },
];
