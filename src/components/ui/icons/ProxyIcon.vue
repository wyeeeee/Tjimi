<template>
  <g>
    <!-- 主体：代表API网关/代理 -->
    <rect x="3" y="8" width="18" height="8" rx="2" fill="none" stroke="currentColor" stroke-width="2"/>
    
    <!-- 输入连接线 -->
    <path d="M1 12h2" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
    <circle cx="1" cy="12" r="1" fill="currentColor"/>
    
    <!-- 输出连接线 -->
    <path d="M21 12h2" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
    <circle cx="23" cy="12" r="1" fill="currentColor"/>
    
    <!-- 内部流动指示器 -->
    <path d="M7 10l2 2-2 2" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" fill="none"/>
    <path d="M11 10l2 2-2 2" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" fill="none"/>
    <path d="M15 10l2 2-2 2" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" fill="none"/>
    
    <!-- 顶部指示灯 -->
    <circle cx="12" cy="5" r="2" fill="none" stroke="currentColor" stroke-width="1.5"/>
    <circle cx="12" cy="5" r="1" fill="currentColor" opacity="0.7"/>
    
    <!-- 底部状态指示 -->
    <rect x="8" y="19" width="8" height="2" rx="1" fill="currentColor" opacity="0.5"/>
  </g>
</template>