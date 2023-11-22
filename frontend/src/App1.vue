<script setup>
import TextEditor from './components/TextEditor.vue'
import SidebarFTextOpt from './components/SidebarFTextOpt.vue'
import {provide, ref } from 'vue';

async function sendMessage(messages) {
  const uri = 'http://127.0.0.1:8080/api/sentmessages';
  const oaibody = {
    model: "gpt-3.5-turbo",
    messages: messages
  };

  try {
    const resp = await fetch(uri, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(oaibody)
    });
    if (!resp.ok) {
      throw new Error('Network response was not ok! Status: ${response.status}');
    }
    const data = await resp.json();
    return data;
  } catch (err) {
    console.error('[sendMessage] Error sending message:',err);
    return null;
}

const text = ref('上海中山北路76号，那座典雅的上海老房子，是中国共产党第一次全国代表大会的旧址。始建于 1920年秋，这座建筑见证了我党诞生的历史进程。1921年7月23日，这里召开了中共一大，通过了纲领性文件和重要决策；同时选举出了中央领导机构，宣告中国共产党正式成立。在1952年9月，这个中共首次全国代表大会的地址进行了翻修并增设了纪念馆，对公众开放。如今，该场地既是爱国主义教育基地，也是了解中共历史及战争时期人民奋斗史的地方。');

provide('text', text);

const sidebarActive = ref(false);
const sidebarName = ref(String)

function toggleSidebar(name) {
  sidebarActive.value = true;
    sidebarName.value = name;
    // console.log(name);
}
function closeSidebar() {
  sidebarActive.value = false;
    sidebarName.value = null;
}

function is_FTextOpt()  {
    return sidebarName.value === "FTextOpt".toString();
}

function is_TopicDig() {
    return sidebarName.value === 'TopicDig'.toString();
}

</script>

<template>
    <div class="container">
        <div class="main-ui" :class="{ 'is-active': sidebarActive }">
            <header>
                <button class="left-custom-button">
                    <span class="button-icon">
                        <!-- 这里直接插入您提供的 SVG -->
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="none" viewBox="0 0 16 16" class="icon">
                          <path fill="url(#ic-ai-command_svg__a)" fill-rule="evenodd" d="M4.067 1.448a.361.361 0 0 0-.686 0L2.901 2.9l-1.453.481a.361.361 0 0 0 0 .686l1.452.481L3.381 6c.11.33.577.33.686 0l.481-1.452L6 4.067a.361.361 0 0 0 0-.686l-1.452-.48-.481-1.453Zm5.963 1.876c-.243-.77-1.334-.77-1.578 0L7.353 6.801 4.026 7.904c-.757.25-.757 1.32 0 1.571l3.327 1.103 1.1 3.477c.243.771 1.334.771 1.577 0l1.1-3.477 3.327-1.103c.756-.25.756-1.32 0-1.57L11.13 6.8l-1.1-3.477Z" clip-rule="evenodd"></path>
                          <defs>
                            <linearGradient id="ic-ai-command_svg__a" x1="1.338" x2="15.023" y1="5.082" y2="5.082" gradientUnits="userSpaceOnUse">
                              <stop stop-color="#5D51FE"></stop>
                              <stop offset="0.296" stop-color="#A049F7"></stop>
                              <stop offset="0.659" stop-color="#CB4DA4"></stop>
                              <stop offset="0.978" stop-color="#FF8080"></stop>
                            </linearGradient>
                          </defs>
                        </svg>
                      </span>
                    AI 写作
                  </button>
                <div class="right-button" >
                    <button @click="toggleSidebar('FTextOpt')" class="right-custom-button" :class="{'green': is_FTextOpt()}">
                        <span class="button-icon">
                          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="none" viewBox="0 0 16 16" class="icon">
                            <g clip-path="url(#ic-full-text-modify_svg__a)">
                              <path fill="url(#ic-full-text-modify_svg__b)" d="M6.294 9.706a.788.788 0 0 1 0 1.115l-4.458 4.458A.788.788 0 0 1 .72 14.164L5.18 9.706a.788.788 0 0 1 1.115 0ZM8.8 10.745a.788.788 0 0 1 .788.788v1.576a.788.788 0 0 1-1.576 0v-1.576a.788.788 0 0 1 .788-.788Zm3.67-.991 1.115 1.115a.79.79 0 0 1-.555 1.355.788.788 0 0 1-.56-.241l-1.114-1.114a.788.788 0 1 1 1.114-1.115ZM4.467 6.411a.788.788 0 1 1 0 1.576H2.89a.788.788 0 1 1 0-1.576h1.576Zm10.245 0a.788.788 0 1 1 0 1.576h-1.576a.788.788 0 0 1 0-1.576h1.576ZM5.227 2.51l1.114 1.114A.788.788 0 1 1 5.227 4.74L4.112 3.625A.788.788 0 0 1 5.227 2.51Zm8.311-.048a.788.788 0 0 1 0 1.115L12.423 4.69a.788.788 0 0 1-1.114-1.114l1.114-1.115a.786.786 0 0 1 1.115 0ZM8.801.5a.788.788 0 0 1 .788.788v1.576a.788.788 0 0 1-1.576 0V1.288A.788.788 0 0 1 8.801.5Z"></path>
                            </g>
                            <defs>
                              <linearGradient id="ic-full-text-modify_svg__b" x1="0.649" x2="15.498" y1="4.835" y2="4.835" gradientUnits="userSpaceOnUse">
                                <stop stop-color="#5D51FE"></stop>
                                <stop offset="0.296" stop-color="#A049F7"></stop>
                                <stop offset="0.659" stop-color="#CB4DA4"></stop>
                                <stop offset="0.978" stop-color="#FF8080"></stop>
                              </linearGradient>
                              <clipPath id="ic-full-text-modify_svg__a">
                                <path fill="#fff" d="M0 0h16v16H0z"></path>
                              </clipPath>
                            </defs>
                          </svg>
                        </span>
                        全文优化
                      </button>
                      <button @click="toggleSidebar('TopicDig')" class="right-custom-button" :class="{'green': is_TopicDig()}">
                        <span class="button-icon">
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="none" viewBox="0 0 16 16" class="text-_body-4">
                                <path fill="url(#ic-preset-modify_svg__a)" fill-rule="evenodd" d="M10.213 11.248a.615.615 0 0 0-.369.585v.932H6.155v-.932a.615.615 0 0 0-.369-.585C3.928 10.437 2.773 8.6 2.773 6.457A5.233 5.233 0 0 1 8 1.23a5.232 5.232 0 0 1 5.226 5.226c0 2.144-1.154 3.98-3.013 4.792Zm-8.67-4.792c0 2.497 1.286 4.661 3.382 5.744v.565c0 .678.552 1.23 1.23 1.23h3.69c.678 0 1.23-.552 1.23-1.23V12.2c2.096-1.083 3.381-3.247 3.381-5.744A6.464 6.464 0 0 0 8 0a6.464 6.464 0 0 0-6.457 6.456ZM9.537 16H6.462a.615.615 0 0 1 0-1.23h3.075a.615.615 0 0 1 0 1.23Zm.008-11.268a.713.713 0 0 0-1.217-.503L4.943 7.614h2.672l-.951.952A.713.713 0 0 0 7.67 9.574l3.385-3.386H8.383l.953-.952a.713.713 0 0 0 .209-.504Z" clip-rule="evenodd"></path>
                                <defs>
                                  <linearGradient id="ic-preset-modify_svg__a" x1="1.672" x2="14.455" y1="4.624" y2="4.624" gradientUnits="userSpaceOnUse">
                                    <stop stop-color="#5D51FE"></stop>
                                    <stop offset="0.296" stop-color="#A049F7"></stop>
                                    <stop offset="0.659" stop-color="#CB4DA4"></stop>
                                    <stop offset="0.978" stop-color="#FF8080"></stop>
                                  </linearGradient>
                                </defs>
                            </svg>     
                        </span>
                        主题深挖
                      </button>
                </div>
            </header>
            <main>
                <TextEditor />
            </main>
        </div>
        <div class="sidebar font_black" :class="{ 'is-active': sidebarActive }">
          <!-- 退出按钮 -->
          <button @click="closeSidebar" class="close-btn">×</button>  
          <!-- 侧边栏内容 -->
            <div v-if="is_FTextOpt()" >
                <SidebarFTextOpt />
            </div>
            <div v-if="is_TopicDig()">
                <h2>主题深挖</h2>
            </div>
        </div>
    </div>
</template>

<style scoped>

.container {
    width:100vw;
    display: flex;
    transition: transform 0.3s ease-in-out;
}
  
.main-ui {
    flex-grow: 1; /* 让主内容区域占据所有可用空间 */
    transition: margin-right 0.3s ease-in-out;
    /*
    border-top: 2px solid #ddd;  添加2px宽的色描边 
    border-radius: 8px; 设置边角为圆角，8px是一个常见的圆角值，可以根据需要调整 
    */
    margin: 0 20em ;
    background-color:#fff;
  }

.main-ui.is-active {
    margin-right: 35.5vw ; /* 侧边栏宽度 */
}

.sidebar {
    width: 35vw; /* 侧边栏宽度 */
    transform: translateX(100%);
    transition: transform 0.3s ease-in-out;
    position: fixed;
    right: 0;
    top: 50px;
    bottom: 0;
    background: #f8f8f8; /* 侧边栏背景颜色 */
    /* 侧边栏其他样式 */
    /* 描边+优化边角*/
    border: 1px solid blue; /* 添加2px宽的蓝色描边 */
    border-radius: 8px; /* 设置边角为圆角，8px是一个常见的圆角值，可以根据需要调整 */
    background: #f8f8f8; /* 侧边栏背景颜色 */
}

.sidebar.is-active {
    transform: translateX(0);   
}

.close-btn {
  font-size: 1.5rem; /* 调整大小以适应设计 */
  line-height: 1; /* 这有助于更好地对齐'X'符号 */
  width: 30px; /* 按钮的宽度 */
  height: 30px; /* 按钮的高度 */
  position: absolute; /* 相对于侧边栏定位 */
  top: 10px; /* 与侧边栏顶部的距离 */
  right: 10px; /* 与侧边栏右边的距离 */
  border: none; /* 去除边框 */
  background: transparent; /* 透明背景 */
  cursor: pointer; /* 鼠标悬停时显示手型光标 */
  border-radius: 15px; /* 可选: 圆角边框 */
}
.close-btn:hover {
  background-color: #eee; /* 鼠标悬停时的背景颜色 */
}

header {
    display: flex; /* 启用 Flexbox 布局 */
    align-items: center; /* 垂直居中对齐 */
    justify-content: flex-start; /* 水平方向从左边开始对齐 */
    width: 100%; /* header 占据整个页面宽度 */
    height: 40px; /* 设置 header 的高度，确保足够空间放置 logo */
    background-color: #fff; /* 可选：设置背景颜色 */

    padding-top: 20px;
  }

main {
    display: flex; /* 启用 Flexbox 布局 */
    align-items: center; /* 垂直居中对齐 */
    justify-content: flex-start; /* 水平方向从左边开始对齐 */
    width: 100%; /* main 占据整个页面宽度 */
    height: 100%; /* 设置 main 的高度， */
    background-color: #fff; /* 可选：设置背景颜色 */
}


.left-custom-button {
    background-color: white; /* 背景颜色 */
    border: 1px solid #CCCCCC; /* 边框颜色和大小 */
    border-radius: 10px; /* 边框圆角 */
    padding: 5px 10px; /* 内边距 */
    margin-left: 15px; /* 按钮和左侧边缘之间的间距 */
    font-size: 16px; /* 文字大小 */
    font-family: Arial, sans-serif; /* 字体 */
    display: flex; /* 为了使图标和文字垂直居中 */ /* 使用 Flexbox 使子元素水平排列 */
    align-items: center; /* 垂直居中内容 */
    justify-content: center; /* 水平居中内容 */
    gap: 10px; /* 图标和文字之间的间隙 */
    box-shadow: 0 2px 5px rgba(0, 0, 0, 0.2); /* 添加阴影 */
    cursor: pointer; /* 鼠标悬停时显示手型光标 */
    outline: none; /* 移除聚焦时的边框 */
  }

.right-button {
    display: flex; /* 使用 Flexbox 使子元素水平排列 */
  align-items: center; /* 垂直居中子元素 */
  justify-content: flex-end; /* 子元素靠右排列 */
  margin-left: auto; /* 将按钮推到容器的右侧 */
}

.right-custom-button {
    background-color: white; /* 背景颜色 */
    border: 1px solid #CCCCCC; /* 边框颜色和大小 */
    border-radius: 10px; /* 边框圆角 */
    padding: 5px 10px; /* 内边距 */
    margin-right: 15px; /* 按钮和左侧边缘之间的间距 */
    font-size: 16px; /* 文字大小 */
    font-family: Arial, sans-serif; /* 字体 */
    display: flex; /* 为了使图标和文字垂直居中 */
    align-items: center; /* 垂直居中内容 */
    justify-content: center; /* 水平居中内容 */
    gap: 10px; /* 图标和文字之间的间隙 */
    box-shadow: 0 2px 5px rgba(0, 0, 0, 0.2); /* 添加阴影 */
    cursor: pointer; /* 鼠标悬停时显示手型光标 */
    outline: none; /* 移除聚焦时的边框 */
  }
  
  .button-icon {
    display: flex;
    align-items: center;
  }
  
  .icon {
    /* If you want to change the size of the icon, adjust these values */
    width: 1em; /* 1em typically equals the current font size */
    height: 1em;
  }

  

</style>