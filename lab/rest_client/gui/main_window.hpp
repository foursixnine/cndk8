// gui/main_window.hpp

#include <QtWidgets/QMainWindow>
#include <QtWidgets/QPushButton>

class MainWindow : public QMainWindow {
  Q_OBJECT

public:
  //static MainWindow* get_instance();
  MainWindow(QWidget *parent = nullptr);

private slots:
  //static bool instanceFlag;
  //static MainWindow *single;
  void onClick();

private:
  QPushButton *button;
};
