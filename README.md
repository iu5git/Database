# Database
Курс ИУ5 по Базам Данных. PostgreSQL

### Общие положения

- Программное обеспечение. Студенты могут использовать виртуальную машину с Ubuntu 20.04 по ссылке выше или компьютеры в аудиториях университета с Alt Linux

- IDE. PgAdmin на всех ПК, но предпочтительнее DataGrip на личных компьютерах

- Импорт БД MS Access на PostgreSQL. Необходимо выправлять типы, возможно через csv.

### Программное обеспечение 

- Образ виртуальной машины Linux [Ubuntu 20.04](https://github.com/iu5git/Standards/blob/main/Linux/Linux.md) для выполнения заданий курса

- [Инструкция](Docker/README.md) по установке PosgreSQL Docker


### Лабораторная. Основы SQL

Создать БД. Create (типы данных, Primary, Foreign), пара таблиц. Select, CRUD. NULL, NOT NULL

[Методические указания](tutorials/lab_sql)


### Лабораторная. Подробно SELECT

Select, where, join, order by. LEFT JOIN. Агрегированные функции, group by, having. Exists, union. 

Работа со строками и датами по вариантам.

Подзапросы (select, from, where)

[Методические указания](tutorials/lab_select)

### Лабораторная. Представления и DCL

[Методические указания](tutorials/lab_view)

### Лабораторная. Создание приложения

Подключить к приложению, курсоры. Транзакции на примере insert (одна) + update (вторая таблица) и rollback.

* **Вариант 1. C#**

Создание оконного приложения на C# для получения данных и добавления/удаления данных

[Методические указания](tutorials/lab_c%23)

* **Вариант 2. Tauri**

Создание оконного приложения на Tauri для получения данных и добавления/удаления данных

[Методические указания](tutorials/lab_tauri)

* **Вариант 3. Qt**

Создание оконного приложения на Qt для получения данных и добавления/удаления данных

[Методические указания - в планах](tutorials/lab_qt.md)

### Лабораторная. Backup

Загрузка данных из csv. Полная версия бекапа и части данных.

[Методические указания](tutorials/lab_backup)

### Лабораторная. Триггеры и хранимые процедуры

PL/SQL, курсоры, хранимые процедуры, триггеры.

[Методические указания](tutorials/lab_trig_proc)

### Курсовая

По вариантам выполнить следующие пункты курсовой работы:

1. Из набора данных (например IMDb в формате csv) загрузить строки в таблицу-витрину.
2. Проанализировать данные - distinct. Создать таблицы под нужную предметную область, наполнить их данными из витрины.
3. Написать запросы select по нужному варианту с определенными условиями. Запросы, работают долго, потому что full scan
4. Проанализировать план запросов, добавить нужные индексы. Проверить чтобы запросы работали быстро.
5. Написать приложение для получения данных из таблицы по заданным условиям. Добавить функционал по добавлению записей и их удалению через приложение.
6. Администратор системы должен иметь доступ к редактированию дополнительных данных

[Методические указания](Курсовая/course_works.md)

### Дополнительная Лабораторная. Индексы и план запроса

Индексы и план запроса

[Методические указания](tutorials/lab_index.md)

### Дополнительная Лабораторная. Транзакции

### Дополнительные материалы. Чат бот с базой данных

Создание чат-бота на Python для получения данных и добавления/удаления данных

[Методические указания](tutorials/tgbot)

### Команда курса выражает благодарность за помощь в подготовке данного курса
1. Федюкин Данила Антонович
2. Калинников Даниил Игоревич 
3. Оразов Алексей Витальевич
4. Мащенко Елена Игоревна
5. Погосян Сос Левонович
6. Карпов Даниил Константинович
7. Ваксина Ия Романовна
