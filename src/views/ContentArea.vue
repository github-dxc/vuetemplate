<template>
  <div class="content-area">
    <div class="single-layout">
      <keep-alive>
        <component 
          :is="singlePanelComponent"
          v-bind="props"
        />
      </keep-alive>
      
    </div>
  </div>
</template>

<script setup vapor>
import { computed } from 'vue';

const props = defineProps({
  currentMenu: Object,
  userAvatar: String,
  bugList: Array,
  bugMsgs: Array,
  groupMsgs: Array,
  bugTotal: Number,
  enums: Object,
});

const singlePanelComponent = computed(() => {
  const component = props.currentMenu?.component;
  return component;
});

</script>

<style scoped>
/* Content area styles from original file */
.content-area { 
  flex: 1;
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
  overflow: hidden;
}
.single-layout {
  height: 100%;
  display: flex;
  flex-direction: column;
  width: 100%;
}
</style>