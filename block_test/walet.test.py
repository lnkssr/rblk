import requests
import json

# URL вашего API
BASE_URL = "http://127.0.0.1:3000"

def log_response(response):
    """Логгирует ответ от сервера"""
    print(f"Status Code: {response.status_code}")
    try:
        print(f"Response JSON: {response.json()}")
    except json.JSONDecodeError:
        print("Ответ не является JSON.")

def create_wallet():
    """Создает новый кошелек"""
    response = requests.post(f"{BASE_URL}/wallet")
    log_response(response)
    if response.status_code == 201:
        wallet = response.json()
        print(f"Кошелек создан: {wallet}")
        return wallet['address']
    else:
        print(f"Ошибка при создании кошелька: {response.status_code}")
        return None

def get_balance(address):
    """Получает баланс кошелька по адресу"""
    response = requests.get(f"{BASE_URL}/balance/{address}")
    log_response(response)
    if response.status_code == 200:
        balance = response.json()  # Это будет просто число, а не объект
        print(f"Баланс кошелька {address}: {balance}")
        return balance
    else:
        print(f"Ошибка при получении баланса: {response.status_code}")
        return None


def create_transaction(from_address, to_address, amount):
    """Создает транзакцию между кошельками"""
    data = {
        "from": from_address,
        "to": to_address,
        "amount": amount
    }
    response = requests.post(f"{BASE_URL}/transaction", json=data)
    log_response(response)
    if response.status_code == 201:
        print(f"Транзакция создана: {response.json()}")
    else:
        print(f"Ошибка при создании транзакции: {response.status_code}, {response.text}")

def add_block(data, miner_address):
    """Добавляет блок в блокчейн"""
    payload = {
        "data": data,
        "miner_address": miner_address
    }
    response = requests.post(f"{BASE_URL}/block", json=payload)
    log_response(response)
    if response.status_code == 201:
        print(f"Блок добавлен: {response.json()}")
    else:
        print(f"Ошибка при добавлении блока: {response.status_code}, {response.text}")

def check_chain_validity():
    """Проверяет валидность блокчейна"""
    response = requests.get(f"{BASE_URL}/chain/validity")
    log_response(response)
    if response.status_code == 200:
        print("Цепочка валидна.")
    else:
        print(f"Ошибка при проверке валидности цепочки: {response.status_code}")

def save_chain():
    """Сохраняет блокчейн"""
    response = requests.post(f"{BASE_URL}/chain/save")
    log_response(response)
    if response.status_code == 200:
        print("Цепочка успешно сохранена.")
    else:
        print(f"Ошибка при сохранении цепочки: {response.status_code}")

def load_chain():
    """Загружает блокчейн"""
    response = requests.post(f"{BASE_URL}/chain/load")
    log_response(response)
    if response.status_code == 200:
        print("Цепочка успешно загружена.")
    else:
        print(f"Ошибка при загрузке цепочки: {response.status_code}")

def run_tests():
    """Запускает тестирование функционала"""
    print("Запуск тестов...")
    wallet_1 = create_wallet()
    wallet_2 = create_wallet()

    if wallet_1 and wallet_2:
        get_balance(wallet_1)
        get_balance(wallet_2)

        # Тестируем транзакцию
        create_transaction(wallet_1, wallet_2, 10)
        get_balance(wallet_1)  # Проверяем баланс после транзакции
        get_balance(wallet_2)

        # Создаем блок
        add_block("Тестовые данные блока", wallet_1)

        # Проверяем валидность блокчейна
        check_chain_validity()

        # Сохраняем и загружаем цепочку
        save_chain()
        load_chain()

if __name__ == "__main__":
    run_tests()
