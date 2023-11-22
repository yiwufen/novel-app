<script setup>

import { ref } from 'vue'
const activeTab = ref('优化设置')
const tabs = ref(['优化设置', '优化结果'])
const selectedOption = ref('');
const dropdownVisible = ref(false);
const options = ref([
    { value: 'novel', text: '小说/散文' },
    { value: 'poetry', text: '诗歌/戏剧' },
    { value: 'academic', text: '工作类' },
    { value: 'report', text: '高端类' },
    { value: 'business', text: '自媒体/创作类' },
    { value: 'other', text: '生活类' },
])
const style_buttons = ref([
    { id: 'formal', text: '正式', active: true },
    { id: 'casual', text: '随意', active: false },
    { id: 'academic', text: '学术', active: false },
    { id: 'other', text: '其他', active: false },
]);
const maxStyleButtons = 1;
const opt_weight_buttons = ref([
  {id: 'finechage', text:'润色', active: false},
  {id: 'change', text:'改写', active: false},
  {id: 'bigchange', text:'二创', active: false},
])
const maxOptWeightButtons = 1;
const optResult = ref('');

function selectOption(option) {
    this.selectedOption.value = option.value;
    this.dropdownVisible.value = false;
}
function toggleStyleButton(buttonId) {
    const button = this.style_buttons.find(b => b.id === buttonId);
    if (button) {
      if (button.active) {
          button.active = false;
        } else {
          // 计算当前激活的按钮数量
          const activeCount = this.style_buttons.filter(b => b.active).length;
          // 如果激活的按钮数量小于最大允许的数量，则激活按钮
          if (activeCount < this.maxStyleButtons) {
            button.active = true;
          }
        }
    }
    // 根据需求，这里可能还需要执行更多的逻辑
}
function toggleOptWeightButton(buttonId) {
  const button = this.opt_weight_buttons.find(b => b.id === buttonId);
    if (button) {
      if (button.active) {
          button.active = false;
        } else {
          // 计算当前激活的按钮数量
          const activeCount = this.opt_weight_buttons.filter(b => b.active).length;
          // 如果激活的按钮数量小于最大允许的数量，则激活按钮
          if (activeCount < this.maxOptWeightButtons) {
            button.active = true;
          }
        }
    }
    // 根据需求，这里可能还需要执行更多的逻辑
}

function applyOptimization() {
  this.activeTab = '优化结果';
}

</script>

<template>
<div class="sidebar-container">
    <div class="font_black title">全文优化</div>
    <div class="tabs">
        <div
            class="tab"
            v-for="(tab, index) in tabs"
            :key="index"
            :class="{ 'active': activeTab === tab }"
            @click="activeTab = tab"
        >
            {{ tab }}
            <span v-if="activeTab === tab" class="underline"></span>
        </div>
    </div>

    <!-- 选项卡内容 -->
    <div class="tab-content">
        <div v-if="activeTab === '优化设置'" class="content">
            <!-- 优化设置的内容 -->
            <div class="font_black">
              <div class="title">优化目标</div>
              <div class="setting-opt">
                <div>你正在写：</div>
                <div class="dropdown">
                  <select v-model="selectedOption">
                    <option disabled value="">请选择</option>
                    <option v-for="option in options" :key="option.value" :value="option.value">
                      {{ option.text }}
                    </option>
                  </select>
                  <ul v-show="dropdownVisible">
                    <li v-for="option in options" :key="option.value" @click="selectOption(option)">
                      {{ option.text }}
                      <span class="arrow">></span>
                    </li>
                  </ul>
                </div>
              </div>
              <div class="setting-opt">
                <div>优化风格</div>
                <div class="button-group">
                  <button
                    v-for="button in style_buttons"
                    :key="button.id"
                    :class="{ 'active': button.active }"
                    @click="toggleStyleButton(button.id)"
                  >
                    {{ button.text }}
                  </button>
                </div>
              </div>
              <div class="setting-opt">
                <div>优化幅度</div>
                <div class="button-group">
                  <button
                    v-for="button in opt_weight_buttons"
                    :key="button.id"
                    :class="{ 'active': button.active }"
                    @click="toggleOptWeightButton(button.id)"
                  >
                    {{ button.text }}
                  </button>
                </div>
              </div>
              
            </div>
            <div class="setting-actions">
                <button class="action-btn" @click="applyOptimization()">全文优化</button>
                <button class="action-btn" @click="cancelOptimization">取消</button>
            </div>
        </div>
        <div v-if="activeTab === '优化结果'" class="content">
            <!-- 优化结果的内容 -->
            <div class="font_black">{{optResult}}</div>

            <div class="setting-actions">
                <button class="action-btn" @click="applyChange">应用</button>
                <button class="action-btn" @click="cancelOptimization">取消</button>
            </div>
        </div>
    </div>
</div>

</template>

<style scoped>
.sidebar-container{
    margin-left: 30px;
}
.title {
    margin-top: 20px;
    height: 40px;
    /*容器内部文本上下对齐*/
    padding: auto;
    font-weight: bold; /*字体加粗*/
}
.tabs {
    display: flex;
    font-size: 16px;
  }
  
  .tab {
    cursor: pointer;
    position: relative;
    padding: 10px;
    margin-right: 10px; /* 选项卡间距 */
  }
  
  .tab .underline {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    height: 2px;
    background-color: purple; /* 下划线颜色 */
  }
  
  .tab.active {
    font-weight: bold; /* 当前选中的选项卡字体加粗 */
  }

  .content {
    position: relative;
    min-height: 70vh;
  }

  .setting-actions {
    display: flex;
    /*添加内margin*/
    padding-top: 15px;

    /*添加上描边*/
    border-top: 1px solid #ddd;

    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;

  }
  .action-btn {
    padding: 10px 20px;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    background-color: #6200EE;
    color: white;
    text-align: center;
  }
  
  .action-btn:not(:last-child) {
    margin-right: 10px;
  }

  .setting-opt {
    display: relative;
    padding: 10px;
  }

  .dropdown {
    position: relative;
    display: inline-block;
  }
  
  .dropdown select {
    width: 100%;
    padding: 10px;
    border: 1px solid #ccc;
    border-radius: 4px;
    appearance: none; /* 移除原生下拉箭头 */
    background-color: white;
  }
  
  .dropdown ul {
    list-style-type: none;
    padding: 0;
    margin: 0;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    position: absolute;
    width: 100%;
    background-color: white;
    border-radius: 4px;
    border: 1px solid #ccc;
    z-index: 100;
  }
  
  .dropdown li {
    padding: 10px;
    border-bottom: 1px solid #eee;
    cursor: pointer;
  }
  
  .dropdown li:last-child {
    border-bottom: none;
  }
  
  .dropdown li:hover {
    background-color: #f6f6f6;
  }
  
  .arrow {
    float: right;
  }

  .button-group {
    display: flex;
  }
  
  .button-group button {
    padding: 8px 16px;
    margin: 5px;
    border: 1px solid #dcdcdc;
    border-radius: 16px;
    background-color: white;
    cursor: pointer;
    outline: none; /* 去掉焦点时的轮廓线 */
    transition: background-color 0.3s, color 0.3s; /* 平滑颜色过渡 */
  }
  
  .button-group button.active {
    color: white;
    background-color: #6200ee;
    border-color: #6200ee;
  }
  
  /* 鼠标悬停效果 */
  .button-group button:not(.active):hover {
    background-color: #f5f5f5;
  }
</style>
