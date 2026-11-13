# мозговой краб
Discord-бот с открытым кодом, написанный на Rust.<br>
Сделан для конкретного discord-сервера, но вы всё также<br>
можете переделать его под свой сервер, эксперементировать<br>
и как либо менять код, пока это не нарушает MIT лицензию.

## Сборка
Перед сборкой создайте `.cargo/config.toml`:
```toml
[env]
BRAIN_CRAB_TOKEN = "ТОКЕН_ВАШЕГО_БОТА"
```
###### (не забудьте добавить его в .gitignore)

Затем введите команду сборки:
```sh
cargo build --release
```

## Что делать, если хостинг не имеет Rust?
Хостинг предоставляет только вонючий Python,<br>
а вы не хотите тратиться на VPS? Не беда!

Создайте python-файл:
```py
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
```
###### скачать готовый можно из корня репозитория (`start.py`)
Назначьте его как входная точка и положите бинарник бота рядом с этим файлом.<br>
И всё! Теперь при каждом запуске просто срабатывает python-скрипт, который запускает бота на Rust.
