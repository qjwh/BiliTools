import { http } from '@/utils';
import { NavResponse, StatResponse } from './types';

export async function getNav(opt: { unwrap: true }): Promise<NavResponse['data'] | null>;
export async function getNav(): Promise<NavResponse | null>;
export async function getNav(opt?: { unwrap?: boolean }): Promise<NavResponse | NavResponse['data'] | null> {
  return await http.biliRequest<NavResponse>({
    url: 'https://api.bilibili.com/x/web-interface/nav',
    method: 'GET',
    type: 'json',
    ...(opt?.unwrap && {
      unwrap: {
        code: 'common'
      }
    })
  });
}

export async function getNavStat(opt: { unwrap: true }): Promise<StatResponse['data'] | null>;
export async function getNavStat(): Promise<StatResponse | null>;
export async function getNavStat(opt?: { unwrap?: boolean }): Promise<StatResponse | StatResponse['data'] | null> {
  return await http.biliRequest<StatResponse>({
    url: 'https://api.bilibili.com/x/web-interface/nav/stat',
    method: 'GET',
    type: 'json',
    ...(opt?.unwrap && {
      unwrap: {
        code: 'common'
      }
    })
  });
}
