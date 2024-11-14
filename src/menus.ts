import { MenuProps } from "ant-design-vue";
import { menuProps } from "ant-design-vue/es/menu/src/Menu";
// export const Menus = [
//   {
//     title: "目录1",
//     icon: "",
//     subMenu: [
//       {
//         title: "子目录1",
//         link: "nothing",
//         icon: "",
//       },
//     ],
//   },
//   {
//     title: "生成二维码",
//     link: "generateQrCode",
//   },
// ];

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
    ],
  },
  {
    type: "group",
    label: "测试",
    children: [
      {
        label: "Markdown渲染测试",
        key: "testMarkDown",
      },
    ],
  },
];
