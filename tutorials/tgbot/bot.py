import telebot
from telebot import types
import numpy as np

import app, db_processor

#ниже указать ранее сгенерированный bot_father'ом токен вашего бота и обновить креды к своей базе
bot = telebot.TeleBot('5322826035:AAEbIzVH-sIh4xNcaoUzhxEaxbxlmuj0fNI')
db_conn = db_processor.DBConection('localhost', '5432', 'db_demand', 'postgres', '12345678')

@bot.message_handler(commands=['start'])
def start_message(message):
	bot.send_message(message.chat.id, 'Привет!')

@bot.message_handler(content_types='text')
def message_reply(message):
    global db_conn
    cmnd = message.text.split(': ')
    if message.text=="/help":
        bot.send_message(message.chat.id, """
        Отправьте сообщение в формате: 
        [Записать/Прочитать/Удалить]: [текст]
        \n
        Подробнее:
        1) Для записи в части текст отправьте запрос формата: INSERT INTO table (col1, col2) VALUES (%s, %s)| val1| val2
        1) Для удаления в части текст отправьте запрос формата: DELETE FROM table Where col=val
        2) Для чтения в части текст отправьте запрос формата: SELECT * FROM table
        """)
    elif cmnd[0]=="Записать":
        bot.send_message(message.chat.id, app.writedb(db_conn, cmnd[1]))
    elif cmnd[0]=="Прочитать":
        bot.send_document(chat_id=message.chat.id, document=app.readdb(db_conn, cmnd[1]))
    elif cmnd[0]=="Удалить":
        bot.send_message(message.chat.id, app.deletedb(db_conn, cmnd[1]))
        

bot.infinity_polling()