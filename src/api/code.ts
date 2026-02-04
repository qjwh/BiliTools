import { AppError } from "@backend";

export const unknownError: AppError = { source: 'common', reason: 'unknown' }

export const commonCodeMap = new Map<number, AppError>([
  [-101, { source: 'common', reason: 'account_not_logined' }],
  [-102, { source: 'common', reason: 'account_banned' }],
  [-111, { source: 'common', reason: 'invalid_csrf' }],
  [-352, { source: 'common', reason: 'risk_verify_failed' }],
  [-400, { source: 'common', reason: 'bad_request' }],
  [-401, { source: 'common', reason: 'unauthorized' }],
  [-403, { source: 'common', reason: 'permission_denied' }],
  [-404, { source: 'common', reason: 'not_found' }],
  [-412, { source: 'common', reason: 'request_rejected' }],
  [-500, { source: 'common', reason: 'server_error' }],
  [-688, { source: 'common', reason: 'geo_limit' }],
  [-699, { source: 'common', reason: 'copyright_limit' }],
  [-799, { source: 'common', reason: 'rate_limited' }],
])
