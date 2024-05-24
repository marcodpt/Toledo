GET http://localhost:8005
HTTP 200
Content-Type: application/json; charset=UTF-8
{"tare":0.0,"unit":"Kg","weight":0.0}

GET http://localhost:8005/
HTTP 500
Content-Type: text/plain; charset=UTF-8
`Weight is not within acceptable limits.`

GET http://localhost:8005#test
HTTP 500
Content-Type: text/plain; charset=UTF-8
`Weight is not within acceptable limits.`

GET http://localhost:8005/#test
HTTP 500
Content-Type: text/plain; charset=UTF-8
`Weight is not within acceptable limits.`

GET http://localhost:8005
HTTP 500
Content-Type: text/plain; charset=UTF-8
`Data passed through the serial port is corrupt.`

GET http://localhost:8005
HTTP 500
Content-Type: text/plain; charset=UTF-8
`The scale is moving.`

GET http://localhost:8005
HTTP 500
Content-Type: text/plain; charset=UTF-8
`Weight above the scale limit.`

GET http://localhost:8005
HTTP 500
Content-Type: text/plain; charset=UTF-8
`Weight is not within acceptable limits.`

GET http://localhost:8005
HTTP 200
Content-Type: application/json; charset=UTF-8
{"tare":6.0,"unit":"Kg","weight":-0.89}

GET http://localhost:8005
HTTP 500
Content-Type: text/plain; charset=UTF-8
`Weight is not within acceptable limits.`
