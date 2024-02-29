import http from 'k6/http';
import { sleep } from 'k6';
import { check, fail } from 'k6';


export const options = {
    executor: 'ramping-arrival-rate',
    stages: [
        { duration: '1m', target: 300 },
    ],
};

export default () => {
    const res = http.get('http://localhost:3000/');
    if (
        !check(res, {
            'Checking for status code 200': (res) => res.status == 200,
        })
    ) {
        fail(`HTTP request failed. Received status ${res.status}`);
    }
    // wait for 0.5 sec after each iteration
    sleep(1)
};