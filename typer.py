#!/usr/bin/python3

from pynput.keyboard import Key, Controller
#import keyboard
from time import sleep
from sys import argv

controller = Controller()

def input_chr(char):
    global controller
    controller.press(char)
    controller.release(char)

def vim_go_to_line(line):
    input_chr(Key.esc)
    input_chr(':')
    for c in f"{line}":
        input_chr(c)
    input_chr(Key.enter)
    input_chr('0')

def vim_newline(above):
    # Exit insert
    input_chr(Key.esc)
    # Make new line
    if above:
        input_chr('O')
    else:
        input_chr('o')
    # Start at beginning of line
    input_chr(Key.esc)

def vim_removeline():
    input_chr('d')
    input_chr('d')

def action_remove(change, pause):
    char = 'k' if change.above else 'j'
    for line in range(change.lines):
        input_chr(char) 
        vim_removeline()
        sleep(pause)

def action_add(change, pause):
    if change.start is not None:
        vim_newline(change.above)
    for line in change.lines:
        input_chr('i')
        for c in line:
            input_chr(c)
            if c == ' ':
                sleep(pause/2)
            else:
                sleep(pause)
        input_chr(Key.esc)
        vim_newline(False)
        sleep(pause)

def action_append_end(change, pause):
    for line in change.lines:
        input_chr(Key.end)
        input_chr('a')
        for c in line:
            input_chr(c)
            if c == ' ':
                sleep(pause/2)
            else:
                sleep(pause)
        input_chr(Key.esc)
        input_chr('j')
        sleep(pause)
        
def action_append_start(change, pause):
    for line in change.lines:
        input_chr(Key.home)
        input_chr('i')
        for c in line:
            input_chr(c)
            if c == ' ':
                sleep(pause/2)
            else:
                sleep(pause)
        input_chr(Key.esc)
        input_chr('j')
        sleep(pause)
        
def vim_type(changes, pause):
    for change in changes:
        if change.start is not None:
            vim_go_to_line(change.start)
            sleep(0.25)
        if change.action == '+':
            action_add(change, pause)
        elif change.action == '-':
            action_remove(change, pause)
        elif change.action == ')':
            action_append_end(change, pause)
        elif change.action == '(':
            action_append_start(change, pause)

def countdown(time):
    print('Starting in ')
    for i in range(time, 0, -1):
        print(i)
        sleep(1)

class Change:
    def __init__(self, above, lines, action, start):
        self.above = above   # True or False
        self.lines = lines   # List of Strings or List of line numbers
        self.action = action # "+" or "-"
        self.start = start   # line number

def multiple_actions_check(action, checking):
    if action is not None and action is not checking:
        print("ERROR: Can't have multiple actions in the same section")
        exit(1)

def parse_file():
    if len(argv) < 2:
        print('ERROR: No file specified')
        exit(1)
    # Read File
    with open(argv[1], 'rt') as f:
        content = f.read().rstrip('\n')
    # Parse File
    changes = []
    for change_section in content.split('----'):
        lines = None
        above = True
        action = None
        start = None
        for line in change_section.split('\n'):
            if len(line) == 0:
                continue
            # Start from line
            if line[0] == '@':
                if lines == None:
                    above = False
                start = int(line[1:])
            # Add
            elif line[0] == '+':
                multiple_actions_check(action, '+')
                action = '+'
                if lines == None:
                    lines = []
                lines.append(line[1:])
            # Append End
            elif line[0] == ')':
                multiple_actions_check(action, ')')
                action = ')'
                if lines == None:
                    lines = []
                lines.append(line[1:])
            # Append Start
            elif line[0] == '(':
                multiple_actions_check(action, '(')
                action = '('
                if lines == None:
                    lines = []
                lines.append(line[1:])
            # Remove
            elif line[0] == '-':
                multiple_actions_check(action, '-')
                action = '-'
                if lines == None:
                    lines = 0
                lines += 1
        changes.append(Change(above, lines, action, start))
    return changes

changes= parse_file()
#for change in changes:
#    print('Change:')
#    print('\tAbove: ', change.above)
#    print('\tLines: ', change.lines)
#    print('\tAction: ', change.action)
#    print('\tStart: ', change.start)
#exit(1)

countdown(5)

vim_type(changes, 0.05) 
