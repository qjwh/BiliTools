import { IconHomeSearch, IconSettings } from '@tabler/icons-vue';

import { SidebarItem } from './types';

export const MENU: SidebarItem[] = [
  {
    key: 'home',
    path: '/',
    icon: IconHomeSearch,
  },
  {
    key: 'settings',
    path: '/settings',
    gapped: true,
    icon: IconSettings,
  },
];
