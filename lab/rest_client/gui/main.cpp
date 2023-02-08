// gui/main.cpp

#include "main_window.hpp"
#include <QtWidgets/QApplication>

int main(int argc, char **argv) {
  QApplication app(argc, argv);

  MainWindow *mainWindow;
  mainWindow = MainWindow::get_instance();
  mainWindow->show();

  app.exec();
}
