GET http://localhost:8007
HTTP 200
Content-Type: application/json; charset=UTF-8
{"tare":0.0,"unit":"Kg","weight":0.0}

GET http://localhost:8007/
HTTP 200
Content-Type: application/json; charset=UTF-8
{"tare":0.0,"unit":"Kg","weight":5.0}

GET http://localhost:8007#test
HTTP 200
Content-Type: application/json; charset=UTF-8
{"tare":0.0,"unit":"Kg","weight":5.67}

GET http://localhost:8007/#test
HTTP 200
Content-Type: application/json; charset=UTF-8
{"tare":0.0,"unit":"Kg","weight":7.1000000000000005}

GET http://localhost:8007
HTTP 500
Content-Type: text/plain; charset=UTF-8
`Data passed through the serial port is corrupt.`

GET http://localhost:8007
HTTP 500
Content-Type: text/plain; charset=UTF-8
`The scale is moving.`

GET http://localhost:8007
HTTP 500
Content-Type: text/plain; charset=UTF-8
`Weight above the scale limit.`

GET http://localhost:8007
HTTP 200
Content-Type: application/json; charset=UTF-8
{"tare":0.0,"unit":"Kg","weight":5.1000000000000005}

GET http://localhost:8007
HTTP 500
Content-Type: text/plain; charset=UTF-8
`The tare was not adjusted correctly when weighing.`

GET http://localhost:8007
HTTP 500
Content-Type: text/plain; charset=UTF-8
`The tare was not adjusted correctly when weighing.`
