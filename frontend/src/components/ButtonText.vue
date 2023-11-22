<script setup>
  import { defineProps, defineEmits } from 'vue';
  
  const props = defineProps({
    options: Array
  });
  
  const emit = defineEmits(['option-selected']);
  
  const emitOption = (option) => {
    emit('option-selected', option);
  };
  </script>

<template>
    <div class="button-text-container font_black center-text">
      <ul>
        <li v-for="option in options" :key="option" @click="emitOption(option)" >
          {{ option }}
        </li>
      </ul>
    </div>
  </template>
  
  
  <style scoped>
  .button-text-container {
    background: #fff;
    border: 1px solid #ddd;
    border-radius: 8px;
    box-shadow: 0 4px 6px rgba(0,0,0,0.1);
    width: 200px;
    /* 其他样式 */
  }
  
  .button-text-container ul {
    list-style-type: none;
    margin: 0;
    padding: 0;
  }
  
  .button-text-container li {
    padding: 10px;
    border-bottom: 1px solid #eee;
    cursor: pointer;
  }
  
  .button-text-container li:last-child {
    border-bottom: none;
  }
  
  .button-text-container li:hover {
    background-color: #f0f0f0;
  }
  </style>
  