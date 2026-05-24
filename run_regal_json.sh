#!/usr/bin/env bash
set -euo pipefail

# Issue the same json_summarize CLI call that test_amc_data_summary.py uses,
# but directly from bash.

cargo run -q -p json_summarize -- \
  'https://www.regmovies.com/api/getShowtimes?theatres=1929,1320,1412,1335,1665,1333,1486,0688,1143,1472,1159,0297,1348,1419,1273,0558,0370,1319&date=11-28-2025&hoCode=&ignoreCache=false&moviesOnly=true' \
  -H 'accept: */*' \
  -H 'accept-language: en-US,en;q=0.9' \
  -b 'RoktRecogniser=76031a4f-8588-43cd-8ba3-152d9d886d11; cf_clearance=gVMQmKBVTemOLlxEAcQx0cKM2BX6UfJWQcYjEYD7BJg-1735339872-1.2.1.1-1xChy03u_m2lSSQwZPksUMtvCwygVc5HEziJVW.FQy.DObmgo1yt0dEqWbemmyR7wEiK1qI2J98VhTeOA6DUm7n_Z51ErzQudMDy7TIS91uEbnDPIICFv9CtkyP1lbKs1g1piYenjj42kWZyW1aLldz1ocSZj44qdWTsR6CL3sFyhzLTLqBZMwVpfchFCL7tTuV8Xr6fpTvxWosF5hiS_jCyJENV9cz0KDbODvdcnTZ3_Xj6.vi0rXxwJEJJavf0qQVRl29ayzaEfUccXsKlvML_D_rJ3kjf275SLy02EdyjyBPAnC4cZi6iGfWGuqaPkfJeyw5pUhBzOzQxAbQymlW0CnfvYhDZ0L37z2mEMStKT5fEGKW0E_t_TZqttOOb0455f9ZeQCBHaXggpX69wkJ6mgC6V_T2YpeJohc6MyRexnzg5DmheZTRn_EaW0WZ; __cf_bm=eSLKZDPUJgar9Y5PPbuS8QS5z35YtzdilL8t5SQVkLU-1764367752-1.0.1.1-N2uECWY8ZWAkmuRg58d4bjxg6q.wmU7r1m05HBwxUVxtGNo43sAEO3kdrnSdDFZ6HsIQuuX4E0wDXLSgJEenh9N0wPIOGDyp_OpxoNg96c4; _cfuvid=dqJRVud5QjGxzaaGTPugDydFjbyFd28cdxe_q0l5Icc-1764367752531-0.0.1.1-604800000; isLoggedIn=false' \
  -H 'dnt: 1' \
  -H 'priority: u=1, i' \
  -H 'referer: https://www.regmovies.com/theatres' \
  -H 'sec-ch-ua: "Not_A Brand";v="99", "Chromium";v="142"' \
  -H 'sec-ch-ua-arch: "arm"' \
  -H 'sec-ch-ua-bitness: "64"' \
  -H 'sec-ch-ua-full-version: "142.0.7444.176"' \
  -H 'sec-ch-ua-full-version-list: "Not_A Brand";v="99.0.0.0", "Chromium";v="142.0.7444.176"' \
  -H 'sec-ch-ua-mobile: ?0' \
  -H 'sec-ch-ua-model: ""' \
  -H 'sec-ch-ua-platform: "macOS"' \
  -H 'sec-ch-ua-platform-version: "15.5.0"' \
  -H 'sec-fetch-dest: empty' \
  -H 'sec-fetch-mode: cors' \
  -H 'sec-fetch-site: same-origin' \
  -H 'user-agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36'
