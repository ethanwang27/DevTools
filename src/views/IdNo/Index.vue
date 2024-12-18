<script setup lang="ts">
import { FormInstance, Rule } from "ant-design-vue/es/form";
import { ref, reactive, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { message } from "ant-design-vue";
import dayjs from "dayjs";

interface IPersonInfo {
  province: string | null;
  city: string | null;
  district: string | null;
  birthday: string | null;
  gender: "Male" | "Female" | null;
  id_no: string | null;
}

interface IState {
  personInfo: IPersonInfo;
  randomDivision: boolean;
  randomBirthDay: boolean;
  num: number;
  provinceData: Array<IProvince>;
  idNoList: Array<IPersonInfo>;
  showSpinning: boolean;
}

interface IProvince extends IDivision {
  cities: Array<ICity>;
}
interface ICity extends IDivision {
  districts: Array<IDivision>;
}

interface IDivision {
  name: string;
  code: string;
}

const state = reactive<IState>({
  randomBirthDay: true,
  randomDivision: true,
  num: 1,
  personInfo: {
    province: null,
    city: null,
    district: null,
    birthday: "",
    gender: null,
    id_no: null,
  },
  provinceData: [],
  idNoList: [],
  showSpinning: false,
});

const checkDivision = async (rule: Rule, value: string) => {
  if (!state.randomDivision && !value) {
    return Promise.reject(rule.message);
  } else {
    return Promise.resolve();
  }
};

const formRef = ref<FormInstance>();
const fromRules: Record<string, Rule[]> = {
  province: [
    {
      required: true,
      trigger: "blur",
      message: "请选择省份",
      validator: checkDivision,
    },
  ],
  city: [
    {
      required: true,
      trigger: "blur",
      message: "请选择市",
      validator: checkDivision,
    },
  ],
  district: [
    {
      required: true,
      trigger: "blur",
      message: "请选择区县",
      validator: checkDivision,
    },
  ],
  birthday: [
    {
      required: true,
      trigger: "change",
      validator: async (_rule: Rule, value: string) => {
        if (state.randomBirthDay === false && !value)
          return Promise.reject("请选择出生日期");
        else Promise.resolve();
      },
    },
  ],
};
const labelCol = { span: 6 };
const wrapperCol = { span: 18 };

const cityData = computed(() => {
  if (state.personInfo.province) {
    return (
      state.provinceData.find(
        (province) => province.name === state.personInfo.province
      )?.cities ?? []
    );
  }
  return [];
});

const districtData = computed(() => {
  if (state.personInfo.city) {
    return (
      state.provinceData
        .find((province) => province.name == state.personInfo.province)
        ?.cities.find((city) => city.name == state.personInfo.city)
        ?.districts ?? []
    );
  }
  return [];
});

function mapToSelectOption(data: IDivision) {
  return {
    label: data.name,
    value: data.name,
  };
}

function reset() {
  state.idNoList = [];
  state.num = 1;
  state.randomBirthDay = true;
  state.randomDivision = true;
  state.personInfo = {
    province: null,
    city: null,
    district: null,
    birthday: "",
    gender: null,
    id_no: null,
  };
}

function copyToClipboard() {
  writeText(state.idNoList.map((info) => info.id_no).join(";"))
    .then(() => message.success("复制成功"))
    .catch((err) => message.error(`复制失败：${err}`));
}

function generateIds() {
  state.idNoList = [];
  if (!formRef.value) return;
  formRef.value.validate().then(() => {
    state.showSpinning = true;
    let person = {
      province: state.personInfo.province,
      city: state.personInfo.city,
      district: state.personInfo.district,
      birthday: state.personInfo.birthday
        ? dayjs(state.personInfo.birthday).format("YYYY-MM-DD HH:mm:ss")
        : "",
      gender: state.personInfo.gender,
    };
    const getIdFunc = Array.from({ length: state.num }, () =>
      invoke("get_id_no", {
        person: person,
      })
    );
    Promise.all(getIdFunc)
      .then((res) => {
        state.idNoList = res as Array<IPersonInfo>;
        state.showSpinning = false;
      })
      .catch((err) => {
        message.error(`获取身份证号码信息失败：${err}`);
        state.showSpinning = false;
      });
  });
}

function getProvinceInfo() {
  invoke("get_all_provinces")
    .then((data) => {
      state.provinceData = data as Array<IProvince>;
    })
    .catch((err) => message.error(`获取省份信息错误：${err}`));
}

onMounted(() => getProvinceInfo());
</script>
<template>
  <div class="main-container">
    <a-form
      ref="formRef"
      :model="state"
      :label-col="labelCol"
      :wrapper-col="wrapperCol"
      :rules="fromRules"
      class="form-container"
    >
      <a-form-item label="出生地" key="randomDivision">
        <a-radio-group v-model:value="state.randomDivision">
          <a-radio-button :value="true">随机</a-radio-button>
          <a-radio-button :value="false">指定出生地</a-radio-button>
        </a-radio-group>
      </a-form-item>
      <a-row>
        <a-col span="6" />
        <a-col span="18">
          <a-form-item label="" key="division">
            <a-space>
              <a-select
                v-model:value="state.personInfo.province"
                style="width: 120px"
                :options="
                  state.provinceData.map((pro) => mapToSelectOption(pro))
                "
                :disabled="state.randomDivision"
              ></a-select>
              <a-select
                v-model:value="state.personInfo.city"
                style="width: 120px"
                :options="cityData.map((city) => mapToSelectOption(city))"
                :disabled="state.randomDivision"
              ></a-select>
              <a-select
                v-model:value="state.personInfo.district"
                style="width: 120px"
                :options="
                  districtData.map((district) => mapToSelectOption(district))
                "
                :disabled="state.randomDivision"
              ></a-select>
            </a-space>
          </a-form-item>
        </a-col>
      </a-row>
      <a-form-item label="出生日期" key="randomBirthday">
        <a-space>
          <a-radio-group v-model:value="state.randomBirthDay">
            <a-radio-button :value="true">随机</a-radio-button>
            <a-radio-button :value="false">指定日期</a-radio-button>
          </a-radio-group>
          <a-date-picker
            v-model:value="state.personInfo.birthday"
            placeholder="请选择出生日期"
            format="YYYY年MM月DD日"
            :disabled="state.randomBirthDay"
          />
        </a-space>
      </a-form-item>
      <a-form-item label="性别" key="gender">
        <a-radio-group v-model:value="state.personInfo.gender">
          <a-radio :value="null">随机</a-radio>
          <a-radio value="Male">男</a-radio>
          <a-radio value="Female">女</a-radio>
        </a-radio-group>
      </a-form-item>
      <a-form-item label="生成数量" key="num">
        <a-input-number v-model:value="state.num" :max="100" />
      </a-form-item>
    </a-form>
    <div class="action">
      <t-button
        type="primary"
        danger
        @click="reset"
        :disabled="state.showSpinning"
        >重置</t-button
      >
      <t-button
        type="primary"
        :disabled="state.idNoList.length === 0 || state.showSpinning"
        @click="copyToClipboard"
        >复制</t-button
      >
      <t-button
        type="primary"
        @click="generateIds"
        :disabled="state.showSpinning"
        >生成</t-button
      >
    </div>
    <a-divider dashed class="divider" />

    <div class="result-container">
      <a-spin
        tip="身份证号生成中……"
        size="large"
        :spinning="state.showSpinning"
        style="height: 100%; width: 100%"
      />
      <a-list v-for="(item, index) in state.idNoList" :key="index">
        <a-row>
          <a-col span="6" class="title">身份证号码：</a-col>
          <a-col span="18" class="content">{{ item.id_no }}</a-col>
        </a-row>
        <div class="remark">
          <a-row>
            <a-col span="6" class="title">出生地：</a-col>
            <a-col span="18" class="content">{{
              `${item.province}${item.city}${item.district}`
            }}</a-col>
          </a-row>
          <a-row>
            <a-col span="6" class="title">出生日期：</a-col>
            <a-col span="18" class="content">{{
              item.birthday?.substring(0, 10)
            }}</a-col>
          </a-row>
          <a-row>
            <a-col span="6" class="title">性别：</a-col>
            <a-col span="18" class="content">{{
              item.gender === "Male" ? "男" : "女"
            }}</a-col>
          </a-row>
        </div>
        <a-divider dashed class="divider" />
      </a-list>
    </div>
    <!-- </a-spin> -->
  </div>
</template>
<style lang="less" scoped>
@import url("/src/style/common.less");

.form-container {
  margin-top: 10px;
  height: 190px;
  .ant-form-item {
    margin: 5px 0;
  }
}
.divider {
  margin: 3px 0;
}

.result-container {
  height: calc(100% - 200px - @action-height);
  overflow: auto;

  .title {
    text-align: right;
  }
  .content {
    text-align: left;
  }
  .remark {
    font-size: 0.8em;
    color: gray;
  }
}
</style>
