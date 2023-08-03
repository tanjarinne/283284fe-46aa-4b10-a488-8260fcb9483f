import http from 'k6/http';
import { sleep, check } from 'k6';

export const options = {
  stages: [
    {duration: '1s', target: 20},
    {duration: '1s', target: 200},
    {duration: '1s', target: 1000},
  ],
};

export default function () {
  const url = 'http://localhost:8000';

  const new_url_payload = JSON.stringify({
    url: "https://tanjarinne.myportfolio.com/work"
  });

  const post_response = http.post(`${url}/url`, new_url_payload);
  check(post_response, {
    'POST request is status 200': (r) => r.status === 200,
  });

  const get_response = http.get(`${url}/48c2922c2`, {redirects: 0});
  check(get_response.headers, {
    'URL redirect is as expected': (r) => r["Location"].includes("tanjarinne.myportfolio.com/work"),
  });
}
