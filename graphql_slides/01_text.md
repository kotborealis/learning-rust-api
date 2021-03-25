# GraphQL {data-background-image="1_-zPjDUowPUWGi91l7d0KOA.png" style="color: black"}

<style>
  pre sc {
    color: red;
  }
  pre n {
    color: #8dcffc;
  }
  pre t {
    color: orange;
  }
</style>

# 

<pre>
<span style="color: red">type</span> GraphQL {
  <span style="color: green">что</span>
  <span style="color: #8dcffc">почему</span>
  <span style="color: orange">как</span>
}
</pre>

# 

<b>Что такое GraphQL?</b>
<h3>"Рантайм и язык запросов для вашего API"</h3>

##

* <p class="fragment">Запросы GraphQL -> Ответы JSON
* <p class="fragment">Спецификация языка
* <p class="fragment">Описание схемы
* <p class="fragment">Запросы к схеме
* <p class="fragment">Реализация сервера/клиента

##

<b>Описание схемы</b>

<pre>
<sc>type</sc> User {
  <n>id</n>: <t>Int</t>
  <n>username</n>: <t>String</t>
  <n>first_name</n>: <t>String</t>
  <n>last_name</n>: <t>String</t>
  <n>friends</n>: <t>[User]</t>
}
</pre>

## 

<b>Описание схемы</b>

* <p class="fragment">Интерфейсы, Unions, Enums
* <p class="fragment">Поля, списки
* <p class="fragment">Скалярные значения: <pre class="fragment"><t>String</t>, <t>Float</t>, <t>Int</t>, <t>Boolean</t>, <t>ID</t>, <t>...</t></pre> </p>

##

<b>Операции</b>

<span style="display: flex; flex-direction: row;">
<pre style="padding: 20px;" class="fragment">
<sc>query</sc> {
  search(<n>q</n>: "name") {
    <n>title</n>
    <n>author</n>
  }
}
</pre>
<pre style="padding: 20px;" class="fragment">
<sc>mutation</sc> {
  create(<n>title</n>: "My fancy book") {
    <n>id</n>
  }
}
</pre>
<pre style="padding: 20px;" class="fragment">
<sc>subscription</sc> {
  on_create() {
    <n>id</n>
    <n>title</n>
  }
}
</pre>
</span>

# 

<b>Почему GraphQL?</b>

## 

<b>Развитие GraphQL</b>

* <p class="fragment">2012 --- внутреннее использование в Facebook
* <p class="fragment">2015 --- OpenSource
* <p class="fragment">2018 --- GraphQL Foundation

## 

<b>Что учесть при создании API?</b>

* <p class="fragment">Эффективность
* <p class="fragment">Документация
* <p class="fragment">Real-time
* <p class="fragment">Интеграция

## 

<b>Эффективность</b>

* <p class="fragment">Overfetching --- слишком много данных, выкидываем часть запроса
* <p class="fragment">Underfetching --- слишком мало данных, делаем ещё запрос
* <p class="fragment"><i>Exactfetching</i> (GraphQL) --- всё, что нужно, ни больше ни меньше

## {data-background-color="white"}

<img src="./VRyV7Jh.png" style="height: 600px; width: auto"/>

## {data-background-color="white"}

<img src="./z9VKnHs.png" style="height: 600px; width: auto"/>

## 

<b>Документация</b>

* <p class="fragment">GraphQL генерирует документацию из схемы
* <p class="fragment">Документация доступна по самому API

## {data-background-color="white"}

<img src="./0_WWBkI3Ptvq7JVdSl.png"/>

## 

<b>Real-time</b>

Пользователи хотят получать данные в реальном времени.

## {data-background-color="white"}

<img src="./1_DOjFWMhLFKYa7yEbYOAzFA.png" style="height: 400px; width: auto"/>

## 

<b>Интеграция систем</b>

GraphQL позволяет интегрировать разные системы в единый источник данных.

## {data-background-color="white"}

<img src="zQggcSX.png" style="height: 600px; width: auto"/>

# Демо

# Итоги

## 

* <p class="fragment">GraphQL даёт:
  * <p class="fragment">Запросы
  * <p class="fragment">Подписки
  * <p class="fragment">Схему
  * <p class="fragment">Документацию
* <p class="fragment">Всё это можно сделать на REST
* <p class="fragment">Но GraphQL даёт удобные инструменты 