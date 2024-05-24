GET http://localhost:8006
HTTP 500
Content-Type: text/plain; charset=UTF-8
`A tara não foi ajustada corretamente na pesagem.`

GET http://localhost:8006/
HTTP 500
Content-Type: text/plain; charset=UTF-8
`A tara não foi ajustada corretamente na pesagem.`

GET http://localhost:8006#test
HTTP 500
Content-Type: text/plain; charset=UTF-8
`A tara não foi ajustada corretamente na pesagem.`

GET http://localhost:8006/#test
HTTP 500
Content-Type: text/plain; charset=UTF-8
`A tara não foi ajustada corretamente na pesagem.`

GET http://localhost:8006
HTTP 500
Content-Type: text/plain; charset=UTF-8
`Dados passado pela porta serial corrompidos.`

GET http://localhost:8006
HTTP 500
Content-Type: text/plain; charset=UTF-8
`A balança está em movimento.`

GET http://localhost:8006
HTTP 500
Content-Type: text/plain; charset=UTF-8
`Peso acima do limite da balança.`

GET http://localhost:8006
HTTP 500
Content-Type: text/plain; charset=UTF-8
`A tara não foi ajustada corretamente na pesagem.`

GET http://localhost:8006
HTTP 200
Content-Type: application/json; charset=UTF-8
{"tare":6.0,"unit":"Kg","weight":-0.89}

GET http://localhost:8006
HTTP 200
Content-Type: application/json; charset=UTF-8
{"tare":6.0,"unit":"Kg","weight":3.67}
