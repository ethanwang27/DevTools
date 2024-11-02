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
];
