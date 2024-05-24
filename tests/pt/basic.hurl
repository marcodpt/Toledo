GET http://localhost:8002
HTTP 200
Content-Type: application/json; charset=UTF-8
{"tare":0.0,"unit":"Kg","weight":0.0}

GET http://localhost:8002/
HTTP 200
Content-Type: application/json; charset=UTF-8
{"tare":0.0,"unit":"Kg","weight":5.0}

GET http://localhost:8002#test
HTTP 200
Content-Type: application/json; charset=UTF-8
{"tare":0.0,"unit":"Kg","weight":5.67}

GET http://localhost:8002/#test
HTTP 200
Content-Type: application/json; charset=UTF-8
{"tare":0.0,"unit":"Kg","weight":7.1000000000000005}

GET http://localhost:8002
HTTP 500
Content-Type: text/plain; charset=UTF-8
`Dados passado pela porta serial corrompidos.`

GET http://localhost:8002
HTTP 500
Content-Type: text/plain; charset=UTF-8
`A balança está em movimento.`

GET http://localhost:8002
HTTP 500
Content-Type: text/plain; charset=UTF-8
`Peso acima do limite da balança.`

GET http://localhost:8002
HTTP 200
Content-Type: application/json; charset=UTF-8
{"tare":0.0,"unit":"Kg","weight":5.1000000000000005}

GET http://localhost:8002
HTTP 200
Content-Type: application/json; charset=UTF-8
{"tare":6.0,"unit":"Kg","weight":-0.89}

GET http://localhost:8002
HTTP 200
Content-Type: application/json; charset=UTF-8
{"tare":6.0,"unit":"Kg","weight":3.67}

GET http://localhost:8002/path
HTTP 404
Content-Type: text/plain; charset=UTF-8
``

POST http://localhost:8002
HTTP 404
Content-Type: text/plain; charset=UTF-8
``

PUT http://localhost:8002
HTTP 404
Content-Type: text/plain; charset=UTF-8
``

DELETE http://localhost:8002
HTTP 404
Content-Type: text/plain; charset=UTF-8
``

PATCH http://localhost:8002
HTTP 404
Content-Type: text/plain; charset=UTF-8
``

HEAD http://localhost:8002
HTTP 404
Content-Type: text/plain; charset=UTF-8
``

GET http://localhost:8002?x=13
HTTP 404
Content-Type: text/plain; charset=UTF-8
``

GET http://localhost:8002?z=dog#cat
HTTP 404
Content-Type: text/plain; charset=UTF-8
``
