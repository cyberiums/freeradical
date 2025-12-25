import http from 'k6/http';
import { check, sleep } from 'k6';

// Load test scenarios for FreeRadical CMS

export let options = {
    stages: [
        { duration: '30s', target: 100 },   // Ramp up to 100 users
        { duration: '1m', target: 100 },    // Stay at 100 users
        { duration: '30s', target: 1000 },  // Ramp up to 1000 users
        { duration: '2m', target: 1000 },   // Stay at 1000 users
        { duration: '30s', target: 0 },     // Ramp down
    ],
    thresholds: {
        http_req_duration: ['p(95)<500'],   // 95% of requests under 500ms
        http_req_failed: ['rate<0.01'],     // Less than 1% failures
    },
};

const BASE_URL = __ENV.BASE_URL || 'http://localhost:8000';

export default function () {
    // Test homepage
    let res = http.get(BASE_URL);
    check(res, {
        'homepage status 200': (r) => r.status === 200,
        'homepage load time < 200ms': (r) => r.timings.duration < 200,
    });

    sleep(1);

    // Test API health endpoint
    res = http.get(`${BASE_URL}/api/health`);
    check(res, {
        'health check 200': (r) => r.status === 200,
    });

    sleep(1);

    // Test GraphQL endpoint
    const query = JSON.stringify({
        query: '{ pages { uuid page_title } }'
    });
    
    res = http.post(`${BASE_URL}/graphql`, query, {
        headers: { 'Content-Type': 'application/json' },
    });
    
    check(res, {
        'GraphQL query 200': (r) => r.status === 200,
        'GraphQL response time < 500ms': (r) => r.timings.duration < 500,
    });

    sleep(2);
}
