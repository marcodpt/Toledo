GET http://localhost:8006
HTTP 500
Content-Type: text/plain; charset=UTF-8
`The tare was not adjusted correctly when weighing.`

GET http://localhost:8006/
HTTP 500
Content-Type: text/plain; charset=UTF-8
`The tare was not adjusted correctly when weighing.`

GET http://localhost:8006#test
HTTP 500
Content-Type: text/plain; charset=UTF-8
`The tare was not adjusted correctly when weighing.`

GET http://localhost:8006/#test
HTTP 500
Content-Type: text/plain; charset=UTF-8
`The tare was not adjusted correctly when weighing.`

GET http://localhost:8006
HTTP 500
Content-Type: text/plain; charset=UTF-8
`Data passed through the serial port is corrupt.`

GET http://localhost:8006
HTTP 500
Content-Type: text/plain; charset=UTF-8
`The scale is moving.`

GET http://localhost:8006
HTTP 500
Content-Type: text/plain; charset=UTF-8
`Weight above the scale limit.`

GET http://localhost:8006
HTTP 500
Content-Type: text/plain; charset=UTF-8
`The tare was not adjusted correctly when weighing.`

GET http://localhost:8006
HTTP 200
Content-Type: application/json; charset=UTF-8
{"tare":6.0,"unit":"Kg","weight":-0.89}

GET http://localhost:8006
HTTP 200
Content-Type: application/json; charset=UTF-8
{"tare":6.0,"unit":"Kg","weight":3.67}
