import { commands } from '@backend';

import { result } from '.';
import { sign } from '@/auth';

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
};

export async function biliRequest<T>(
  opt: BiliRequestOpt & {
    type: 'json';
  },
): Promise<T>;

export async function biliRequest(
  opt: BiliRequestOpt & {
    type: 'text';
  },
): Promise<string>;

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

  switch (resp.type) {
    case 'json':
      return JSON.parse(resp.data);
    case 'text':
      return resp.data;
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
