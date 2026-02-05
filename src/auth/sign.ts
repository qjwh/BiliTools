import { getNav } from '@/api/auth';
import { http } from '@/utils';

import CryptoJS from 'crypto-js';

export function biliUuid(now: string) {
  const o = (length: number) =>
    Array.from({ length }, () =>
      Math.ceil(Math.random() * 16)
        .toString(16)
        .toUpperCase(),
    )
      .join('')
      .padStart(length, '0');

  const s = String(now).padStart(5, '0');

  return `${o(8)}-${o(4)}-${o(4)}-${o(4)}-${o(12)}${s}`;
}

export function biliTicketHexSign(message: string, key: string) {
  const vv = (input: string) => {
    let r = '';
    for (let i = 0; i < input.length; i++) {
      r += String.fromCharCode(input.charCodeAt(i) - 1);
    }
    return r;
  };

  const buffer = CryptoJS.HmacSHA256(message, vv(key));
  return CryptoJS.enc.Hex.stringify(buffer);
}

export async function wbi(query: http.types.JsonValue) {
  const navResp = await getNav();
  if (!navResp) return query;
  const nav = navResp.data;

  const { img_url, sub_url } = nav.wbi_img;

  const imgKey = img_url
    .substring(img_url.lastIndexOf('/') + 1, img_url.length)
    .split('.')[0];

  const subKey = sub_url
    .substring(sub_url.lastIndexOf('/') + 1, sub_url.length)
    .split('.')[0];

  const key = imgKey + subKey;
  const map = [
    46, 47, 18, 2, 53, 8, 23, 32, 15, 50, 10, 31, 58, 3, 45, 35, 27, 43, 5, 49,
    33, 9, 42, 19, 29, 28, 14, 39, 12, 38, 41, 13, 37, 48, 7, 16, 24, 55, 40,
    61, 26, 17, 0, 1, 60, 51, 30, 4, 22, 25, 54, 21, 56, 59, 6, 63, 57, 62, 11,
    36, 20, 34, 44, 52,
  ];
  const chars: string[] = [];
  map.forEach((i) => {
    const char = key.charAt(i);
    if (char) chars.push(char);
  });
  const derived = chars.join('').slice(0, 32);

  const now = Math.round(Date.now() / 1e3);
  const filter = /[!'()*]/g;

  const params = Object.assign({}, query, {
    wts: now,
  });

  const pair = Object.keys(params).sort();
  const parts: string[] = [];
  for (const key of pair) {
    let value = params[key];
    if (value && typeof value === 'string') {
      value = value.replace(filter, '');
    }
    if (value !== null && value !== undefined) {
      parts.push(`${encodeURIComponent(key)}=${encodeURIComponent(value)}`);
    }
  }

  const w_rid = CryptoJS.MD5(parts.join('&') + derived).toString();

  return {
    ...query,
    w_rid,
    wts: now.toString(),
  };
}
