import { commands } from '@backend';

import { result } from '.';
import { sign } from '@/auth';
import * as ApiCode from '@/api/code';

export type JsonValue = Record<
  string,
  string | number | boolean | null | undefined
>;

type BiliRequestOpt = {
  url: string;
  method: 'GET' | 'POST';
  query?: {
    wbi?: boolean;
    data: JsonValue;
  };
  body?: {
    type: 'json' | 'form';
    data: JsonValue;
  };
  unwrap?: {
    code: 'common'
  }
};

type RespEnvelope<R> =
  | { code: number; message: string; data: R }
  | { code: number; msg: string; data: R }
  | { code: number; message: string; result: R }
  | { code: number; msg: string; result: R };

export function biliRequest<
  T extends RespEnvelope<unknown>,
>(
  opt: BiliRequestOpt & {
    type: 'json';
    unwrap: {
      code: 'common';
    };
  }
): Promise<
  T extends RespEnvelope<infer R> ? R | null: never
>;

export async function biliRequest<T>(
  opt: BiliRequestOpt & {
    type: 'json';
  },
): Promise<T | null>;

export async function biliRequest(
  opt: BiliRequestOpt & {
    type: 'text';
  },
): Promise<string | null>;

export async function biliRequest(
  opt: BiliRequestOpt & {
    type: 'json' | 'text';
  },
) {
  const url = opt.url;
  const method = opt.method;
  const query = opt.query ? await getQuery(opt.query) : null;
  const body = opt.body ? bodyToPair(opt.body) : null;
  const response = opt.type;

  const resp = await result.unwrap(
    commands.biliRequest({
      url,
      method,
      query,
      body,
      response,
    }),
  );

  if (!resp) return;

  switch (resp.type) {
    case 'text': {
      return resp.data;
    }
    case 'json': {
      const json = JSON.parse(resp.data);
      if (!opt.unwrap) return json;

      // SAFE: as the caller meets the type requirements, the code, msg etc. are not null at runtime.
      const code = json.code as number;
      const data = json.data ?? json.result;

      if (code === 0) return data;
      
      result.unwrap(
        { status: 'error', error: getErrorByCode(
          opt.unwrap.code, code
        ) }
      );

      return null;
    }
  }
}

function getErrorByCode(id: 'common', code: number) {
  switch (id) {
    case 'common': return ApiCode.commonCodeMap.get(code) ?? ApiCode.unknownError
  }
}

async function getQuery(query: {
  wbi?: boolean;
  data: JsonValue;
}): Promise<[string, string][]> {
  if (query.wbi) {
    const params = await sign.wbi(query.data);
    return jsonToPair(params);
  } else {
    return jsonToPair(query.data);
  }
}

function jsonToPair(json: JsonValue): [string, string][] {
  if (!json) return [];
  return Object.entries(json).map(([key, value]) => [
    key,
    JSON.stringify(value),
  ]);
}

function bodyToPair(body: { type: 'json' | 'form'; data: JsonValue }):
  | {
      Json: string;
    }
  | {
      Form: [string, string][];
    } {
  switch (body.type) {
    case 'json':
      return {
        Json: JSON.stringify(body.data),
      };
    case 'form':
      return {
        Form: jsonToPair(body.data),
      };
  }
}
