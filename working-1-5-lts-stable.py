import base64
import random

# Try to import pyfiglet for fancy banner text, otherwise use plain text
try:
    import pyfiglet
    def print_banner(figure: str):
        banner = pyfiglet.figlet_format(figure)
        return banner + "\n" + "(version 1.5)"
except ImportError:
    def print_banner(figure: str):
        return figure + "\n" + "(version 1.5)"

# Try to import blessed for coloured output
try:
    from blessed import Terminal
    term = Terminal()
    def print_coloured(text: str, colour: str = "white"):
        print(getattr(term, colour) + text + term.normal)
except ImportError:
    def print_coloured(text: str, colour: str = "white"):
        print(text)

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

def scramble(s):
    """Scramble the string by swapping adjacent characters."""
    scrambled = ''.join(s[i+1] + s[i] if i + 1 < len(s) else s[i] for i in range(0, len(s), 2))
    return scrambled

def unscramble(s):
    """Unscramble the string by swapping adjacent characters back."""
    unscrambled = ''.join(s[i+1] + s[i] if i + 1 < len(s) else s[i] for i in range(0, len(s), 2))
    return unscrambled

def encode_string(s):
    """Encode a string using the advanced method."""
    # Step 0: Check for non-ASCII characters
    if any(ord(c) > 127 for c in s):
        # Use UTF-8 encoding with a special indicator
        utf8_encoded = '~' + base64.b64encode(s.encode('utf-8')).decode()
        return utf8_encoded
    
    # Proceed with ASCII encoding for ASCII-only strings
    # Step 1: Base64 Encode
    base64_encoded = base64_encode(s)
    # Step 2: Scramble
    scrambled = scramble(base64_encoded)
    # Step 3: Base32 Encode
    base32_encoded = base32_encode(scrambled)
    # Step 4: Reverse
    final_encoded = base32_encoded[::-1]
    return final_encoded

def decode_string(s):
    """Decode a string using the advanced method."""
    try:
        # Step 0: Check for the special UTF-8 indicator
        if s.startswith('~'):
            # Decode UTF-8 encoded string
            utf8_decoded = base64.b64decode(s[1:]).decode('utf-8')
            return utf8_decoded

        # Proceed with ASCII decoding
        # Step 4: Reverse
        reversed_s = s[::-1]
        # Step 3: Base32 Decode
        base32_decoded = base32_decode(reversed_s)
        # Step 2: Unscramble
        unscrambled = unscramble(base32_decoded)
        # Step 1: Base64 Decode
        final_decoded = base64_decode(unscrambled)
        return final_decoded
    except Exception:
        return None

def main():
    password = "sigma"
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
        if user_password == password:
            print(term.move_up + term.clear_eol + term.green + user_password + term.normal)
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
                    print_coloured("Exiting the programme. Goodbye!", "bold_red")
                    return
                else:
                    print_coloured("Invalid option. Please choose 'encode', 'decode', or 'exit'.", "bold_red")
        else:
            print(term.move_up + term.clear_eol + term.red + user_password + term.normal)
            print_coloured("Incorrect password. Try again.", "bold_red")
            attempts += 1
    print_coloured("Too many incorrect attempts. Exiting.", "bold_red")

if __name__ == "__main__":
    main()
