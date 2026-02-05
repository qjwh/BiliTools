import { AppError } from "@backend";

const map = new Map<number, AppError>([
  [-101, { source: 'biliApi', reason: 'account_not_logined' }],
  [-102, { source: 'biliApi', reason: 'account_banned' }],
  [-111, { source: 'biliApi', reason: 'invalid_csrf' }],
  [-352, { source: 'biliApi', reason: 'risk_verify_failed' }],
  [-400, { source: 'biliApi', reason: 'bad_request' }],
  [-401, { source: 'biliApi', reason: 'unauthorized' }],
  [-403, { source: 'biliApi', reason: 'permission_denied' }],
  [-404, { source: 'biliApi', reason: 'not_found' }],
  [-412, { source: 'biliApi', reason: 'request_rejected' }],
  [-500, { source: 'biliApi', reason: 'server_error' }],
  [-688, { source: 'biliApi', reason: 'geo_limit' }],
  [-699, { source: 'biliApi', reason: 'copyright_limit' }],
  [-799, { source: 'biliApi', reason: 'rate_limited' }],
])

export const getError =
  (code: number) => map.get(code) ?? { source: 'biliApi', reason: 'unknown' };
