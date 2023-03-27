# @ryanpell/scanner-listener
Keyboard event listener for USB HID scanner

## Install
**node.js**:
```bash
npm install @ryanpell/scanner-listener --save
```

## Usage
```typescript

import * as scanner from '@ryanpell/scanner-listener'

//Start Scanner
scanner.start((data) => console.log("Return:", data));

//Stop Scanner
scanner.stop();

//Add/Remove Listener
scanner.addListener('log', (data: any) => console.log(data))
scanner.removeListener('log')
```

### Start Function
This will start the keyboard listener on the main window element. Only one listener will be allowed to run at once. To start this a return function will need to be set on the parameter when it is called

### Stop Function
This will stop the keyboard listener and clear the function from the window element.

### Add Function
Adding a Function will add the function to the relevant event. If the data event is used then the scanner must be running and will overwrite the existing output function.

### Remove Function
Removing will remove the relevant event. If the data event is called then this will stop if the scanner is running.