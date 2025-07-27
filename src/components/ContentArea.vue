<template>
  <div class="content-area">
    <div class="single-layout">
      <keep-alive>
        <component 
          :is="singlePanelComponent" 
          :current-menu="currentMenu"
          :user-avatar="userAvatar"
        />
      </keep-alive>
      
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue';

const props = defineProps({
  currentMenu: Object,
  userAvatar: String,
});

const singlePanelComponent = computed(() => {
  return props.currentMenu?.component;
});

</script>

<style scoped>
/* Content area styles from original file */
.content-area { flex: 1; display: flex; flex-direction: column; }
.single-layout { height: 100%; display: flex; flex-direction: column; }
</style>