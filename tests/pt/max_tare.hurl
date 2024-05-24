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
`Dados passado pela porta serial corrompidos.`

GET http://localhost:8007
HTTP 500
Content-Type: text/plain; charset=UTF-8
`A balança está em movimento.`

GET http://localhost:8007
HTTP 500
Content-Type: text/plain; charset=UTF-8
`Peso acima do limite da balança.`

GET http://localhost:8007
HTTP 200
Content-Type: application/json; charset=UTF-8
{"tare":0.0,"unit":"Kg","weight":5.1000000000000005}

GET http://localhost:8007
HTTP 500
Content-Type: text/plain; charset=UTF-8
`A tara não foi ajustada corretamente na pesagem.`

GET http://localhost:8007
HTTP 500
Content-Type: text/plain; charset=UTF-8
`A tara não foi ajustada corretamente na pesagem.`
