import { Component } from 'vue';

export type SidebarKey = 'home' | 'settings';

export interface SidebarItem {
  key: SidebarKey;
  path: string;
  gapped?: boolean;
  icon: Component;
}
