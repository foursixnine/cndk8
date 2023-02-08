// gui/main_window.cpp

#include "main_window.hpp"

extern "C" {
void hello_world();
}

void MainWindow::onClick() { 
    // Call the `hello_world` function to print a message to stdout
    hello_world(); 
}

MainWindow::MainWindow(QWidget *parent) : QMainWindow(parent) {
  button = new QPushButton("Click Me", this);
  
  // Connect the button's `released` signal to `this->onClick()`
  connect(button, SIGNAL(released()), this, SLOT(onClick()));
}

MainWindow* MainWindow::get_instance()
{
    if(! instanceFlag)
    {
        single = new MainWindow();
        instanceFlag = true;
        return single;
    }
    else
    {
        return single;
    }
}
