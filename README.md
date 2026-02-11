# Lab para testar cenarios simples de uso da ferramenta Nginx

A ideia e abordar diferentes contextos onde o uso desta ferramenta auxilia no processo de desenvolvimento.

## Tecnologias usadas

[Nginx]() - Web server / Proxy Reverso e Load Balancer  
[Docker]() - Criacao / execucao de containers  
[Typescript]() - Linguagem de programacao  
[Rust]()  - Linguagem de programacao  

## Como Testar (SPA)

Uma Aplicacao Simples React para testar o nginx como servidor estatico

Baixe o Projeto:
```bash
git clone https://github.com/DanielDeAzevedoCordeiro1/nginx-lab.git
```

Acesse a pasta nginx-lab e depois spa:
```bash
cd nginx-lab && cd spa
```

Instale as dependencias:
```bash
npm install
```

Suba seu container com a aplicacao:
```bash
docker compose up -d
```

Acesse o endpoint:
```bash
curl localhost:8000/
```

Conteudo retornado:
```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <link rel="icon" type="image/svg+xml" href="/vite.svg" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>spa</title>
    <script type="module" crossorigin src="/assets/index-uV4twp7Y.js"></script>
    <link rel="stylesheet" crossorigin href="/assets/index-CnI6PBz5.css">
  </head>
  <body>
    <div id="root"></div>
  </body>
</html>

```
### Nota:
> O servidor Nginx retornará um HTML padrão que referencia um arquivo `.js`, o qual será carregado pelo navegador e responsável por renderizar o restante da página.  
> Essa é uma característica comum em aplicações do tipo SPA (*Single Page Application*).
