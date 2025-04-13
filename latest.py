import base64, random, sys, os


# Step 1: Prehash the password "sigma" and store the result as bytes
prehashed_password_bytes = bytes.fromhex("38de90475bb334fb3dea5d54f250500aba60fe2c6158115d342b06bcb46e39bf")

# Step 2: Define a constant to be concatenated
constant = os.urandom(16)


# Try to import pyfiglet for fancy banner text, otherwise use plain text
try:
    import pyfiglet
    def print_banner(figure: str):
        banner = pyfiglet.figlet_format(figure)
        return banner + "\n" + "(version 1.9)"
except ImportError:
    def print_banner(figure: str):
        return figure + "\n" + "(version 1.9)"

# Try to import blessed for coloured output
try:
    from blessed import Terminal
    term = Terminal()
    def print_coloured(text: str, colour: str = "white"):
        print(getattr(term, colour) + text + term.normal)
except (ImportError, AttributeError):
    def print_coloured(text: str, colour: str = "white"):
        print(text)

try:
    import hashlib
except ImportError:
    print_coloured("You don't have hashlib! Install it with 'pip install hashlib'.", "bold_red")
    sys.exit(1) # Exit with errors

def hash_with_constant(prehashed_password, constant):
    """Combine the prehashed password with a constant and hash the result."""
    combined = prehashed_password + constant
    hashed = hashlib.sha256(combined).digest()
    return hashed

def verify_password(stored_password, input_password, constant):
    """Verify a stored password against an input password combined with a constant."""
    # Prehash the user input password first
    user_prehashed_password = hashlib.sha256(input_password.encode('utf-8')).digest()
    combined = user_prehashed_password + constant
    hashed_input_password = hashlib.sha256(combined).digest()
    return hashed_input_password == stored_password


def base64_encode(s):
    """Encode a string to Base 64."""
    return base64.b64encode(s.encode()).decode()

def base64_decode(s):
    """Decode a Base 64 string."""
    return base64.b64decode(s).decode()

def base32_encode(s):
    """Encode a string to Base 32."""
    return base64.b32encode(s.encode()).decode()

def base32_decode(s):
    """Decode a Base 32 string."""
    return base64.b32decode(s).decode()

def hex_encode(s):
    """Encode a string to hex."""
    return s.encode().hex()

def hex_decode(s):
    """Decode a hex string."""
    return bytes.fromhex(s).decode()

def bytes_to_hex_string(data):
    """Convert bytes to a hex string."""
    return ''.join(f'{byte:02x}' for byte in data)

def scramble_1(s):
    """Scramble the string by swapping adjacent characters."""
    scrambled = ''.join(s[i+1] + s[i] if i + 1 < len(s) else s[i] for i in range(0, len(s), 2))
    return scrambled

def unscramble_1(s):
    """Unscramble the string by swapping adjacent characters back."""
    unscrambled = ''.join(s[i+1] + s[i] if i + 1 < len(s) else s[i] for i in range(0, len(s), 2))
    return unscrambled

def scramble_2(s):
    """Scramble the string by converting to hex, reversing, then base64 encoding."""
    hex_encoded = hex_encode(s)
    reversed_hex = hex_encoded[::-1]
    base64_encoded = base64_encode(reversed_hex)
    scrambled = scramble_1(base64_encoded)
    return scrambled

def unscramble_2(s):
    """Unscramble the string by reversing the scramble_2 process."""
    unscrambled = unscramble_1(s)
    base64_decoded = base64_decode(unscrambled)
    reversed_hex = base64_decoded[::-1]
    final_decoded = hex_decode(reversed_hex)
    return final_decoded

def encode_method_1(s):
    """Encode a string by converting to bytes, turning bytes to a hex string, and reversing in pairs."""
    try:
        # Step 1: Convert to bytes
        byte_data = s.encode('utf-8')
        # Step 2: Turn bytes into a hex string
        hex_string = bytes_to_hex_string(byte_data)
        # Step 3: Reverse the hex string in pairs
        reversed_string = ''.join([hex_string[i:i+2] for i in range(0, len(hex_string), 2)][::-1])
        # Step 4: Append ending character
        final_encoded = reversed_string + '!'
        return final_encoded
    except Exception as e:
        print(f'Error in encoding: {e}')
        return None

def decode_method_1(s):
    """Decode a string encoded with encode_method_1."""
    try:
        # Step 1: Remove the '!' character
        s = s[:-1]
        # Step 2: Reverse the string back in pairs
        reversed_string = ''.join([s[i:i+2] for i in range(0, len(s), 2)][::-1])
        # Step 3: Convert the hex string back to bytes
        byte_data = bytes.fromhex(reversed_string)
        # Step 4: Convert bytes to the original string
        original_string = byte_data.decode('utf-8')
        return original_string
    except Exception as e:
        print(f'Error in decoding: {e}')
        return None

def encode_method_2(s):
    """Encode a string by converting to bytes, turning bytes to a hex string, reversing, and Base64 encoding."""
    try:
        # Step 1: Convert to bytes
        byte_data = s.encode('utf-8')
        # Step 2: Turn bytes into a hex string
        hex_string = byte_data.hex()
        # Step 3: Reverse the hex string
        reversed_string = hex_string[::-1]
        # Step 4: Base64 encode the reversed string
        base64_encoded = base64.b64encode(reversed_string.encode('utf-8')).decode('utf-8')
        return base64_encoded + '£'
    except Exception as e:
        print(f'Error in encoding: {e}')
        return None

def decode_method_2(s):
    """Decode a string encoded with encode_method_2."""
    try:
        # Step 1: Remove the '£' character
        s = s[:-1]
        # Step 2: Base64 decode the string with padding correction
        missing_padding = len(s) % 4
        if missing_padding != 0:
            s += '=' * (4 - missing_padding)
        base64_decoded = base64.b64decode(s).decode('utf-8')
        # Step 3: Reverse the string
        reversed_string = base64_decoded[::-1]
        # Step 4: Convert the reversed hex string back to bytes
        byte_data = bytes.fromhex(reversed_string)
        # Step 5: Convert bytes to the original string
        original_string = byte_data.decode('utf-8')
        return original_string
    except Exception as e:
        print(f'Error in decoding: {e}')
        return None

def encode_string(s):
    """Encode a string using a randomly chosen scrambling method."""
    if any(ord(c) > 127 for c in s):
        # Use UTF-8 encoding with a special indicator
        utf8_encoded = '~' + base64.b64encode(s.encode('utf-8')).decode()
        return utf8_encoded

    method = random.choice([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
    # Proceed with the selected scrambling method
    if method == 1:
        # Step 1: Base64 Encode
        base64_encoded = base64_encode(s)
        # Step 2: Scramble
        scrambled = scramble_1(base64_encoded)
        # Step 3: Base32 Encode
        base32_encoded = base32_encode(scrambled)
        # Step 4: Reverse
        final_encoded = base32_encoded[::-1] + '#'
    elif method == 2:
        final_encoded = scramble_2(s) + '~'
    elif method == 3:
        final_encoded = encode_method_1(s)
    elif method == 4:
        final_encoded = encode_method_2(s)
    elif method == 5:
        base64_encoded = base64_encode(s)
        reversed_string = base64_encoded[::-1]
        scrambled = ''.join(reversed_string[i+1] + reversed_string[i] if i + 1 < len(reversed_string) else reversed_string[i] for i in range(0, len(reversed_string), 2))
        hex_encoded = hex_encode(scrambled)
        final_encoded = hex_encoded + '$'
    elif method == 6:
        hex_encoded = hex_encode(s)
        reversed_string = hex_encoded[::-1]
        base64_encoded = base64_encode(reversed_string)
        scrambled = scramble_1(base64_encoded)
        final_encoded = scrambled + '%'
    elif method == 7:
        base32_encoded = base32_encode(s)
        reversed_string = base32_encoded[::-1]
        hex_encoded = hex_encode(reversed_string)
        scrambled = scramble_1(hex_encoded)
        final_encoded = scrambled + '^'
    elif method == 8:
        base64_encoded = base64_encode(s)
        scrambled = scramble_1(base64_encoded)
        base32_encoded = base32_encode(scrambled)
        reversed_string = base32_encoded[::-1]
        final_encoded = reversed_string + '&'
    elif method == 9:
        byte_data = s.encode('utf-8')
        byte_string = repr(byte_data)
        reversed_string = byte_string[::-1]
        scrambled = scramble_1(reversed_string)
        final_encoded = scrambled + '¬'
    elif method == 10:
        byte_data = s.encode('utf-8')
        hex_string = bytes_to_hex_string(byte_data)
        reversed_string = hex_string[::-1]
        base64_encoded = base64_encode(reversed_string)
        final_encoded = base64_encoded + '@'
    return final_encoded
def decode_string(s):
    """Decode a string using the specified unscrambling method based on its ending character."""
    try:
        # Step 0: Check for the special UTF-8 indicator
        if s.startswith('~'):
            # Decode UTF-8 encoded string
            utf8_decoded = base64.b64decode(s[1:]).decode('utf-8')
            return utf8_decoded

        # Determine the method from the ending character
        if s.endswith('#'):
            method = 1
        elif s.endswith('~'):
            method = 2
        elif s.endswith('!'):
            method = 3
        elif s.endswith('£'):
            method = 4
        elif s.endswith('$'):
            method = 5
        elif s.endswith('%'):
            method = 6
        elif s.endswith('^'):
            method = 7
        elif s.endswith('&'):
            method = 8
        elif s.endswith('¬'):
            method = 9
        elif s.endswith('@'):
            method = 10
        else:
            return None
        s = s[:-1]

        # Proceed with the selected unscrambling method
        if method == 1:
            reversed_s = s[::-1]
            base32_decoded = base32_decode(reversed_s)
            unscrambled = unscramble_1(base32_decoded)
            final_decoded = base64_decode(unscrambled)
        elif method == 2:
            final_decoded = unscramble_2(s)
        elif method == 3:
            final_decoded = decode_method_1(s)
        elif method == 4:
            final_decoded = decode_method_2(s)
        elif method == 5:
            hex_decoded = hex_decode(s)
            unscrambled = ''.join(hex_decoded[i+1] + hex_decoded[i] if i + 1 < len(hex_decoded) else hex_decoded[i] for i in range(0, len(hex_decoded), 2))
            reversed_string = unscrambled[::-1]
            final_decoded = base64_decode(reversed_string)
        elif method == 6:
            unscrambled = unscramble_1(s)
            base64_decoded = base64_decode(unscrambled)
            reversed_string = base64_decoded[::-1]
            final_decoded = hex_decode(reversed_string)
        elif method == 7:
            unscrambled = unscramble_1(s)
            hex_decoded = hex_decode(unscrambled)
            reversed_string = hex_decoded[::-1]
            final_decoded = base32_decode(reversed_string)
        elif method == 8:
            reversed_string = s[::-1]
            base32_decoded = base32_decode(reversed_string)
            unscrambled = unscramble_1(base32_decoded)
            final_decoded = base64_decode(unscrambled)
        elif method == 9:
            unscrambled = unscramble_1(s)
            reversed_string = unscrambled[::-1]
            byte_data = eval(reversed_string)
            final_decoded = byte_data.decode('utf-8')
        elif method == 10:
            base64_decoded = base64_decode(s)
            reversed_string = base64_decoded[::-1]
            byte_data = bytes.fromhex(reversed_string)
            final_decoded = byte_data.decode('utf-8')
        return final_decoded
    except Exception as e:
        print(f'Error: {e}')
        return None
def main():
    stored_password = hash_with_constant(prehashed_password_bytes, constant)
    attempts = 0
    decode_attempts = 0

    quirky_messages = [
        "Well, that didn't go as planned!",
        "Maybe try a different approach?",
        "Maybe you've got a typo. Check your decoder string."
    ]

    roast_messages = [
        "That's invalid... just like your life.",
        "You call that an attempt? Try harder!",
        "Don't you have anything better to do with your life than mess with a decoder system?\nOh yeah...you don't have one, do you?"
    ]

    print_coloured(print_banner("BASE/Scramble"), "bold_yellow")
    while attempts < 3:
        print_coloured("Enter password: ", "bold_cyan")
        user_password = input()
        if verify_password(stored_password, user_password, constant):
            try:
                print(term.move_up + term.clear_eol + term.green + user_password + term.normal)
            except:
                print("Correct password!")
            while True:
                print_coloured("Do you want to encode, decode, or exit? ", "white")
                action = input().strip().lower()
                if action == "encode":
                    print(term.move_up + term.clear_eol + term.yellow + action + term.normal)
                    print_coloured("Enter the text you want to encode: ", "bold_yellow")
                    text = input().strip()
                    encoded_text = encode_string(text)
                    print_coloured(f"Encoded: {encoded_text}", "bold_cyan")
                elif action == "decode":
                    print(term.move_up + term.clear_eol + term.yellow + action + term.normal)
                    print_coloured("Enter the text you want to decode: ", "bold_yellow")
                    text = input().strip()
                    decoded_text = decode_string(text)
                    if decoded_text is None:
                        print_coloured("That's invalid.", "bold_red")
                        decode_attempts += 1
                        if decode_attempts == 3:
                            print_coloured(random.choice(quirky_messages), "bold_yellow")
                        elif decode_attempts == 5:
                            print_coloured(random.choice(roast_messages), "bold_yellow")
                            decode_attempts = 0  # Reset after showing roast messages
                    else:
                        print_coloured(f"Decoded: {decoded_text}", "bold_cyan")
                        decode_attempts = 0  # Reset on successful decode
                elif action == "exit":
                    print(term.move_up + term.clear_eol + term.red + action + term.normal)
                    print_coloured("Exiting the program. Goodbye!", "bold_red")
                    sys.exit(0)
                    return 0
                else:
                    print_coloured("Invalid option. Please choose 'encode', 'decode', or 'exit'.", "bold_red")
        else:
            try:
                print(term.move_up + term.clear_eol + term.red + user_password + term.normal)
            except:
               print("Enter answer below.\n")
            print_coloured("Incorrect password. Try again.", "bold_red")
            attempts += 1
    print_coloured("Too many incorrect attempts. Exiting.", "bold_red")

if __name__ == "__main__":
    main()
