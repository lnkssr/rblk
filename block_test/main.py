import requests
import json

BASE_URL = "http://127.0.0.1:3000"

def log_response(response):
    print(f"Status Code: {response.status_code}")
    try:
        print(f"Response JSON: {response.json()}")
    except json.JSONDecodeError:
        print("Response body is not JSON.")

def test_create_wallet():
    response = requests.post(f"{BASE_URL}/wallet")
    log_response(response)
    if response.status_code == 201:
        wallet = response.json()
        print(f"Создан кошелек: {wallet}")
        return wallet['address']
    else:
        print(f"Ошибка при создании кошелька: {response.status_code}")
        return None

def test_get_balance(address):
    response = requests.get(f"{BASE_URL}/balance/{address}")
    log_response(response)
    if response.status_code == 200:
        balance = response.json()
        print(f"Баланс кошелька {address}: {balance}")
    else:
        print(f"Ошибка при получении баланса кошелька {address}: {response.status_code}")

def test_create_transaction(from_address, to_address, amount):
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

def test_add_block(data, miner_address):
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

def test_check_chain_validity():
    response = requests.get(f"{BASE_URL}/chain/validity")
    log_response(response)
    if response.status_code == 200:
        print("Цепочка валидна.")
    else:
        print(f"Ошибка при проверке валидности цепочки: {response.status_code}")

def test_save_chain():
    response = requests.post(f"{BASE_URL}/chain/save")
    log_response(response)
    if response.status_code == 200:
        print("Цепочка успешно сохранена.")
    else:
        print(f"Ошибка при сохранении цепочки: {response.status_code}")

def test_load_chain():
    response = requests.post(f"{BASE_URL}/chain/load")
    log_response(response)
    if response.status_code == 200:
        print("Цепочка успешно загружена.")
    else:
        print(f"Ошибка при загрузке цепочки: {response.status_code}")

def run_tests():
    print("Запуск тестов...")
    wallet_1 = test_create_wallet()
    wallet_2 = test_create_wallet()

    if wallet_1 and wallet_2:
        test_get_balance(wallet_1)
        test_get_balance(wallet_2)

        # Проверка транзакции с достаточным балансом
        test_create_transaction(wallet_1, wallet_2, 10)
        test_get_balance(wallet_1)  # Проверяем баланс после транзакции
        test_get_balance(wallet_2)

        # Проверка транзакции с недостаточным балансом
        test_create_transaction(wallet_2, wallet_1, 600)  # Пример перевода больше, чем на балансе

        test_add_block("Данные для блока", wallet_1)

        test_check_chain_validity()

        test_save_chain()

        test_load_chain()

if __name__ == "__main__":
    run_tests()
