# Лабораторная работа №2 (Дополнительная)
## Использование представлений в СУБД PostgreSQL.

>**Цель**: Получить теоретические и практические навыки создания представлений данных в СУБД PostgreSQL.

### Содержание лабораторной работы:
1.	Изучить теоретические сведения лабораторной работы.
2.	Создать любое простое представление и запросить с помощью него данные.
3.	Проверить соответствие данных прямым запросом.
4.	Изменить созданное представление с помощью команды ALTER VIEW, добавив
псевдонимы полям и условие.
5.	Вставить данные с помощью представления.
6.	Создать представление с опцией WITH CHECK OPTION.
7.	Удалить представление.
8.	Создать представление на выборку из двух таблиц с помощью редактора.
9.	Защитить лабораторную работу. - Любой вопрос по выполнению лабораторной работы; - Любой контрольный вопрос

### Краткий вспомогательный материал
**Представление (англ. view)** - хранимый в базе данных под отдельным именем запрос на выборку, по сути - это виртуальная таблица. Рассмотрим варианты использования 
представлений.  
- Возврат небольшого поднабора данных таблицы.  
- Предоставление пользователю возможности запроса данных из множества таблиц через одно представление, с точки зрения пользователя обращение будет происходить 
как к обычной таблице.  
- Модификация данных с помощью представлений.  
- Представления могут использоваться как механизмы безопасности, давая возможность пользователям обращаться к данным через представления, но не предоставляя 
им разрешений на непосредственный доступ к таблицам, лежащим в основе представлений.

Рассмотрим упрощенный формат создания представления:
``` SQL
CREATE VIEW <название> [(<список столбцов>)] AS <запрос> [WITH CHECK OPTION]
```
Здесь *< название >* − название представления, *<список столбцов>* − список названий столбцов, *< запрос >* − запрос, на котором основано представление.
WITH CHECK OPTION может указываться для обновляемых (updatable) представлений, через которые можно добавлять или изменять данные.

Для изменения представления в SQL Server используется оператор ***ALTER***:
``` SQL
ALTER VIEW <название> [(<список столбцов>)] AS <запрос>;
```

Удаление представления выполняется с помощью команды ***DROP***:
``` SQL
DROP VIEW <название>
```

**Пример:**
```SQl
--создаем представление
CREATE VIEW vAUTHORS (FIO) AS
SELECT DISTINCT Author FROM BOOKS
--изменяем представление
ALTER VIEW vAUTHORS (AuthFIO) AS SELECT DISTINCT Author
FROM BOOKS
WHERE Publisher =’BHV’
-- удаляем представление
DROP VIEW vAUTHORS
```

В приведенном примере название столбца в представлении будет отличаться от результата выражения *SELECT*. В следующем примере названия меняться не будут.

Создадим еще одно представление и попробуем c его помощью вставить запись.

***Пример:***
```SQL
--создаем представление
CREATE VIEW vBOOKS AS SELECT Bookname, Author, Publisher FROM BOOKS
WHERE Publisher =’BHV’
--вставляем запись
INSERT INTO vBOOKS VALUES (‘Война и мир’,’Л. Толстой’,’BHV’)
--вставляем запись с издательством, отсутствующем в представлении
INSERT INTO vBOOKS VALUES (‘Война и мир’,’Л. Толстой’,’Эксмо’)
```

SQL успешно выполнит оба запроса, т.е. через представление можно внести запись, которая не соответствует условию представления. Чтобы этого не происходило, изменим
представление, добавив в него проверку.

```SQL
ALTER VIEW vBOOKS AS SELECT Bookname, Author, Publisher FROM BOOKS
WHERE Publisher =’BHV’ WITH CHECK OPTION
```

Если представление предполагается использовать для модификации данных, вам следует помнить следующее:
- Если вы применяете представление для модификации данных, то эта модификация может повлиять только на базовую таблицу. Это означает, что если представление
отображает данные из двух таблиц, вы можете написать инструкцию, обновляющую только одну таблицу. Если же некоторая инструкция попытается обновить обе таблицы,
вы получите сообщение об ошибке.
- Вы не можете модифицировать данные в представлении, использующем агрегатные функции (например, SUM() или AVG()). Если вы попытаетесь модифицировать такое
представление, то получите сообщение об ошибке.
-При попытке вставить запись в представление, не отображающее все поля, может возникнуть проблема. Некоторые из полей, не отображаемые в представлении, могут
не принимать пустые значения, вы не можете вставить данные в поля, отсутствующие в представлении.

## Контрольные вопросы
1.	Что такое представление? Когда оно используется?
2.	Опишите формат создания представления.
3.	Каковы возможности по работе с представлениями?
4.	Опишите опцию WITH CHECK OPTION.
5.	С помощью какой команды можно удалить представление?
6.	С помощью какой команды можно изменить представление?
7.	Какие вы знаете ограничения на модификацию данных через представление?
