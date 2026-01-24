import { http } from '@/utils';
import { NavResponse, StatResponse } from './types';

export async function getNav() {
  return await http.biliRequest<NavResponse>({
    url: 'https://api.bilibili.com/x/web-interface/nav',
    method: 'GET',
    type: 'json',
  });
}

export async function getNavStat() {
  return await http.biliRequest<StatResponse>({
    url: 'https://api.bilibili.com/x/web-interface/nav/stat',
    method: 'GET',
    type: 'json',
  });
}
