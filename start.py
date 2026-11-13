import os
import stat
import subprocess

bot_file = "./discord-bot"

# выдаём нашему бинарнику права на исполнение
st = os.stat(bot_file)
os.chmod(bot_file, st.st_mode | stat.S_IEXEC)

# теперь запускаем его
try:
    subprocess.run([bot_file], check=True)
except subprocess.CalledProcessError as e:
    print(f"бот завершил работу с ошибкой (код {e.returncode})")
except KeyboardInterrupt:
    print("\nостановка бота пользователем...")
