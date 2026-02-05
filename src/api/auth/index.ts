import { http } from '@/utils';
import { NavResponse, NavStatResponse } from './types';

export async function getNav() {
  return http.biliRequest<NavResponse>({
    url: 'https://api.bilibili.com/x/web-interface/nav',
    method: 'GET',
    type: 'json',
  });
}

export async function getNavStat() {
  return await http.biliRequest<NavStatResponse>({
    url: 'https://api.bilibili.com/x/web-interface/nav/stat',
    method: 'GET',
    type: 'json',
  });
}
