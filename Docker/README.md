# Рекомендации по установке PostgreSQL в Docker контейнер

Если ваш выбор пал на установку СУБД в docker-container можете использовать данный compose файл.

## Шаги для установки
1. Скопировать compose файл в новую папку
2. Выполнить в терминала команду
 
   ```
   docker-compose up
   ```

Подробнее можно причитать [здесь](https://habr.com/ru/post/578744/).


## Дополнительный вариант развертывания PostgreSQL в Docker контейнер

1. Скачиваем образ из Docker Hub
```console
docker pull postgres
```
2. Запускаем контейнер с СУБД
```console
docker run --name postgres -p 5455:5432 -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres1234 -e POSTGRES_DB=test_db -d postgres
```


