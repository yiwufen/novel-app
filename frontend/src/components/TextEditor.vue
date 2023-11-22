<script setup>
import { ref } from 'vue';
import { inject } from 'vue';

import ButtonText from './ButtonText.vue';

const text = inject('text');
const opi = inject('opi');


// 在 <script setup> 中定义的响应式数据

const showToolbar = ref(false);
const toolbarPosition = ref({ x: 0, y: 0 });

const handleTextSelect = (event) => {
  // 获取 <textarea> 元素
  const textarea = event.target;

  // 检查是否有文本被选中
  if (textarea.selectionStart !== textarea.selectionEnd) {
    // 选中文本存在
    // 获取 <textarea> 的边界矩形
    const rect = textarea.getBoundingClientRect();

    // console.log("scrollX", window.scrollX);
    // console.log("scrollY", window.scrollY);
    // console.log( event.clientX, event.clientY);
    // 获取鼠标指针的位置（视口坐标）
    const mouseX =  -rect.left + window.scrollX + event.clientX;
    const mouseY =  -rect.top + window.scrollY - 50 + event.clientY;
    console.log(mouseX, mouseY);

    // 更新工具栏的显示状态和位置
    toolbarPosition.value = { x: mouseX, y: mouseY };


    showToolbar.value = true;
  } else {
    // 没有选中文本，隐藏工具栏
    showToolbar.value = false;
  }
};

async function sentopi() {
  let resp = await opi.chat.completions.create({
    messages: [{ role: "user", content: "Say this is a test" }],
    model: "gpt-3.5-turbo",
  });
  // console.log(resp);
  return resp;
}

// console.log(await sentopi());


const applyStyle = (style) => {
  document.execCommand(style, false, null);
};
</script>

<template>
  <div class="text-editor">
    <textarea v-model="text" @mouseup="handleTextSelect" class="text-area font_black"></textarea>
    <div v-show="showToolbar" :style="{ top: toolbarPosition.y + 'px', left: toolbarPosition.x + 'px'}" class="toolbar">
      <div>
        <p class="font_black center-text">ai改写</p>
        <ButtonText
        :options="['提升表达', '修改语气', '抄写', '总结', '短句', '翻译']"
        @option-selected="handleOptionSelected"
        />
      </div>
    </div>
  </div>
</template>


<style scoped>
.text-editor {
  width: 100%;
  position: relative;
}
.text-area {
  width: 100%;
  height: 81vh;
  border: none;
  outline: none;
  border-top: 1px solid rgba(0, 0, 0, 0.1);
  padding-top:30px ;
}
.toolbar {
  position: absolute;
  transform: translateX(-50%);
  display: flex;
  gap: 4px;
  background-color: #eee;
  padding: 8px;
  border-radius: 4px;
  z-index: 100;
}
</style>
