export type JsonValue = Record<
  string,
  string | number | boolean | null | undefined
>;

// ===== API Requirements ===== //

type RespEnvelope<R> =
  | { code: number; message: string; data: R }
  | { code: number; msg: string; data: R }
  | { code: number; message: string; result: R }
  | { code: number; msg: string; result: R };

export type UnwrapEnvelope<T> =
  T extends RespEnvelope<infer R> ? R | null : never;

// ===== Request OPT === //

type RequestBaseOpt = {
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

export type RequestJsonOpt = RequestBaseOpt & {
  type: 'json';
  unwrap?: undefined;
};

export type RequestJsonUnwrapOpt = RequestBaseOpt & {
  type: 'json';
  unwrap: true;
};

export type RequestTextOpt = RequestBaseOpt & {
  type: 'text';
}

export type RequestOpt =
  | RequestJsonOpt
  | RequestJsonUnwrapOpt
  | RequestTextOpt;
