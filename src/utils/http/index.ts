import { commands } from '@backend';

import { result } from '@/utils';
import { sign } from '@/auth';

import { getError } from './code';
import * as types from './types';

export { types };

export async function biliRequest<T>(
  opt: types.RequestJsonOpt,
): Promise<T | null>;

export async function biliRequest<T>(
  opt: types.RequestJsonUnwrapOpt,
): Promise<types.UnwrapEnvelope<T>>;

export async function biliRequest(
  opt: types.RequestTextOpt,
): Promise<string | null>;

export async function biliRequest(
  opt: types.RequestOpt,
) {
  const url = opt.url;
  const method = opt.method;
  const query = opt.query ? await getQuery(opt.query) : null;
  const body = opt.body ? bodyToPair(opt.body) : null;

  const resp = await result.unwrap(
    commands.biliRequest({
      url,
      method,
      query,
      body,
    }),
  );

  if (!resp) return;

  switch (opt.type) {
    case 'text': {
      return resp;
    }
    case 'json': {
      const json = JSON.parse(resp);
      if (opt.type === 'json' && (!opt.unwrap)) return json;

      // SAFE: as the caller meets the type requirements, the code, msg etc. are not null at runtime.
      const code = json.code as number;
      const data = json.data ?? json.result;

      if (code === 0) return data;
      
      result.unwrap(
        { status: 'error', error: getError(code) }
      );

      return null;
    }
  }
}

async function getQuery(query: {
  wbi?: boolean;
  data: types.JsonValue;
}): Promise<[string, string][]> {
  if (query.wbi) {
    const params = await sign.wbi(query.data);
    return jsonToPair(params);
  } else {
    return jsonToPair(query.data);
  }
}

function jsonToPair(data: types.JsonValue): [string, string][] {
  if (!data) return [];
  return Object.entries(data).map(([key, value]) => [
    key,
    JSON.stringify(value),
  ]);
}

function bodyToPair(body: { type: 'json' | 'form'; data: types.JsonValue }):
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
